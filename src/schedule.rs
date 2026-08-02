//! Scheduled entity dispatch — cron, one-shot (`at`), and random re-scheduling.
//!
//! ## Schedule types
//!
//! | Variant | Spec format | Behaviour |
//! |---------|------------|-----------|
//! | `Cron` | 6-field cron `"sec min hour day month weekday"` or English spec | Fires on schedule indefinitely. |
//! | `Interval` | Human-readable duration (`"1h"`, `"30m"`, `"5s"`, `"2h30m"`) | Fires every N seconds indefinitely. |
//! | `At` | Unix timestamp in milliseconds | Fires once after the computed delay. |
//! | `Random` | `max_secs: u64` | Fires after a random 1–N second delay, then self-reschedules. |
//!
//! Schedules are registered dynamically by plugins via `ma_send` to `#scheduler`
//! in their `:start` handling. There are no static schedules in `EntityNode`.
//!
//! ## ACL
//!
//! Scheduled dispatch bypasses all ACL checks.  The runtime is the trusted
//! caller.
//!
//! ## Outbound messages
//!
//! Outbound envelopes from scheduled calls are sent fire-and-forget by the
//! `ma_send` host function directly via a channel to the main event loop.
//! The scheduler has no envelope-handling responsibility.
//! State changes via `ma_set_state` are persisted to IPFS normally.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use ma_core::{CONTENT_TYPE_TERM, MESSAGE_TYPE_RPC};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{trace, warn};

use crate::entity::{CastInput, LocalMessage};
use crate::manifest::ManifestWriter;
use crate::plugin::EntityRegistry;

// ── Schedule request ──────────────────────────────────────────────────────────

/// A schedule request enqueued by a plugin via `ma_send` to `#scheduler`.
#[derive(Debug, Clone)]
pub enum ScheduleRequest {
    /// Recurring cron / English schedule.
    Cron {
        spec: String,
        /// Pre-encoded CBOR call bytes (verb atom or `[":verb", arg…]` array).
        content: Vec<u8>,
    },
    /// Fixed-interval recurring dispatch.
    Interval { secs: u64, content: Vec<u8> },
    /// One-shot dispatch at a Unix millisecond timestamp.
    At { timestamp_ms: i64, content: Vec<u8> },
    /// Self-rescheduling one-shot with a random delay up to `max_secs`.
    Random { max_secs: u64, content: Vec<u8> },
}

// ── Scheduler context ─────────────────────────────────────────────────────────

/// Minimal context cloned into every scheduled job closure.
#[derive(Clone)]
pub struct SchedulerCtx {
    pub entity_registry: EntityRegistry,
    pub manifest_writer: Arc<tokio::sync::RwLock<Option<ManifestWriter>>>,
    pub kubo_rpc_url: String,
    pub our_did: String,
}

/// Guard that returns `true` only while a schedule definition is still current.
pub type ActiveScheduleGuard = Arc<dyn Fn() -> bool + Send + Sync>;

// ── Job registration ──────────────────────────────────────────────────────────

/// Register a [`ScheduleRequest`] on the scheduler for the named entity.
///
/// `schedule_id`: an optional opaque identifier for this job, used for
/// logging only.  All registered schedules always dispatch — there is no
/// static schedule map to check against.
///
/// Returns the scheduler-assigned [`uuid::Uuid`] for the created job.
pub async fn register_schedule(
    sched: &JobScheduler,
    ctx: SchedulerCtx,
    fragment: String,
    schedule_id: Option<String>,
    active_guard: Option<ActiveScheduleGuard>,
    req: ScheduleRequest,
) -> Result<uuid::Uuid> {
    let id = match req {
        ScheduleRequest::Cron { spec, content } => {
            let job = Job::new_async(spec.as_str(), {
                let ctx = ctx.clone();
                let fragment = fragment.clone();
                let schedule_id = schedule_id.clone();
                let active_guard = active_guard.clone();
                move |_, _| {
                    let ctx = ctx.clone();
                    let fragment = fragment.clone();
                    let content = content.clone();
                    let schedule_id = schedule_id.clone();
                    let active_guard = active_guard.clone();
                    Box::pin(async move {
                        dispatch_if_active(
                            &ctx,
                            &fragment,
                            schedule_id.as_ref(),
                            active_guard.as_ref(),
                            &content,
                            "cron",
                        )
                        .await;
                    })
                }
            })?;
            sched.add(job).await?
        }

        ScheduleRequest::Interval { secs, content } => {
            let job = Job::new_repeated_async(Duration::from_secs(secs), {
                let ctx = ctx.clone();
                let fragment = fragment.clone();
                let schedule_id = schedule_id.clone();
                let active_guard = active_guard.clone();
                move |_, _| {
                    let ctx = ctx.clone();
                    let fragment = fragment.clone();
                    let content = content.clone();
                    let schedule_id = schedule_id.clone();
                    let active_guard = active_guard.clone();
                    Box::pin(async move {
                        dispatch_if_active(
                            &ctx,
                            &fragment,
                            schedule_id.as_ref(),
                            active_guard.as_ref(),
                            &content,
                            "interval",
                        )
                        .await;
                    })
                }
            })?;
            sched.add(job).await?
        }

        ScheduleRequest::At {
            timestamp_ms,
            content,
        } => {
            add_at_schedule_job(
                sched,
                ctx,
                fragment,
                schedule_id,
                active_guard,
                content,
                timestamp_ms,
            )
            .await?
        }

        ScheduleRequest::Random { max_secs, content } => {
            add_random_schedule_job(
                sched,
                ctx,
                fragment,
                schedule_id,
                active_guard,
                content,
                max_secs,
            )
            .await?
        }
    };
    Ok(id)
}

async fn add_random_schedule_job(
    sched: &JobScheduler,
    ctx: SchedulerCtx,
    fragment: String,
    schedule_id: Option<String>,
    active_guard: Option<ActiveScheduleGuard>,
    content: Vec<u8>,
    max_secs: u64,
) -> Result<uuid::Uuid> {
    let job = make_random_job(
        sched.clone(),
        ctx,
        fragment,
        schedule_id,
        active_guard,
        content,
        max_secs,
    )?;
    sched.add(job).await.map_err(Into::into)
}

async fn add_at_schedule_job(
    sched: &JobScheduler,
    ctx: SchedulerCtx,
    fragment: String,
    schedule_id: Option<String>,
    active_guard: Option<ActiveScheduleGuard>,
    content: Vec<u8>,
    timestamp_ms: i64,
) -> Result<uuid::Uuid> {
    let now_ms = i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )
    .unwrap_or(i64::MAX);
    let delay_ms = u64::try_from((timestamp_ms - now_ms).max(0)).unwrap_or(0);
    let job = Job::new_one_shot_async(Duration::from_millis(delay_ms), {
        let ctx = ctx.clone();
        let fragment = fragment.clone();
        let schedule_id = schedule_id.clone();
        let active_guard = active_guard.clone();
        move |_, _| {
            let ctx = ctx.clone();
            let fragment = fragment.clone();
            let content = content.clone();
            let schedule_id = schedule_id.clone();
            let active_guard = active_guard.clone();
            Box::pin(async move {
                dispatch_if_active(
                    &ctx,
                    &fragment,
                    schedule_id.as_ref(),
                    active_guard.as_ref(),
                    &content,
                    "one-shot",
                )
                .await;
            })
        }
    })?;
    sched.add(job).await.map_err(Into::into)
}

async fn dispatch_if_active(
    ctx: &SchedulerCtx,
    fragment: &str,
    schedule_id: Option<&String>,
    active_guard: Option<&ActiveScheduleGuard>,
    content: &[u8],
    schedule_kind: &str,
) {
    if !active_guard.is_none_or(|is_active| is_active()) {
        trace!(fragment = %fragment, schedule_id = ?schedule_id, "scheduled dispatch skipped: stale {schedule_kind} schedule");
        return;
    }
    dispatch_scheduled(ctx, fragment, schedule_id.map(String::as_str), content).await;
}

/// Create a self-rescheduling one-shot random job.
///
/// After firing, it checks whether the schedule definition is still current
/// before re-adding, so stale generations terminate naturally.
pub fn make_random_job(
    sched: JobScheduler,
    ctx: SchedulerCtx,
    fragment: String,
    schedule_id: Option<String>,
    active_guard: Option<ActiveScheduleGuard>,
    content: Vec<u8>,
    max_secs: u64,
) -> Result<Job> {
    let delay = rand_delay(max_secs);
    Ok(Job::new_one_shot_async(delay, move |_, _| {
        let sched = sched.clone();
        let ctx = ctx.clone();
        let fragment = fragment.clone();
        let schedule_id = schedule_id.clone();
        let active_guard = active_guard.clone();
        let content = content.clone();
        Box::pin(async move {
            dispatch_if_active(
                &ctx,
                &fragment,
                schedule_id.as_ref(),
                active_guard.as_ref(),
                &content,
                "random",
            )
            .await;

            // Re-schedule only while this schedule definition is still current.
            let should_reschedule = active_guard.as_ref().is_none_or(|is_active| is_active());
            if should_reschedule {
                match make_random_job(
                    sched.clone(),
                    ctx,
                    fragment.clone(),
                    schedule_id,
                    active_guard,
                    content,
                    max_secs,
                ) {
                    Ok(next) => {
                        if let Err(e) = sched.add(next).await {
                            warn!(fragment = %fragment, error = %e, "failed to reschedule random job");
                        }
                    }
                    Err(e) => {
                        warn!(fragment = %fragment, error = %e, "failed to create next random job");
                    }
                }
            } else {
                trace!(fragment = %fragment, schedule_id = ?schedule_id, "random schedule chain stopped: superseded by newer definition");
            }
        })
    })?)
}

fn rand_delay(max_secs: u64) -> Duration {
    use rand::Rng;
    let secs = rand::thread_rng().gen_range(1..=max_secs.max(1));
    Duration::from_secs(secs)
}

/// Parse a human-readable duration string into a [`Duration`].
///
/// Supported units: `s` (seconds), `m` (minutes), `h` (hours), `d` (days).
/// Units may be combined: `"1h30m"`, `"90s"`, `"2d12h"`.
pub fn parse_duration(s: &str) -> Result<Duration> {
    let mut total_secs: u64 = 0;
    let mut num_buf = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num_buf.push(ch);
        } else {
            let n: u64 = num_buf
                .parse()
                .with_context(|| format!("invalid number in duration {s:?}"))?;
            num_buf.clear();
            let unit_secs = match ch {
                's' => n,
                'm' => n * 60,
                'h' => n * 3_600,
                'd' => n * 86_400,
                _ => bail!("unknown duration unit {ch:?} in {s:?}"),
            };
            total_secs += unit_secs;
        }
    }
    if !num_buf.is_empty() {
        bail!("duration {s:?} ends with a number but no unit");
    }
    if total_secs == 0 {
        bail!("duration must be > 0");
    }
    Ok(Duration::from_secs(total_secs))
}

// ── Scheduled dispatch ────────────────────────────────────────────────────────

/// Call an entity plugin on schedule, bypassing all ACL checks.
///
/// - Outbound envelopes from the call are silently dropped.
/// - State changes via `ma_set_state` are persisted to IPFS.
pub async fn dispatch_scheduled(
    ctx: &SchedulerCtx,
    fragment: &str,
    schedule_id: Option<&str>,
    content: &[u8],
) {
    let plugin = ctx.entity_registry.read().await.get(fragment).cloned();
    let Some(plugin) = plugin else {
        warn!(fragment = %fragment, "scheduled dispatch: entity not found");
        return;
    };

    // Guard: if this job was registered with a named id, log it for tracing.
    if let Some(id) = schedule_id {
        tracing::trace!(fragment = %fragment, id = %id, "scheduled dispatch firing");
    }

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Produce a stable message ID from the timestamp + fragment without
    // pulling in a uuid dep here — blake3 is already available.
    let mut hasher = blake3::Hasher::new();
    hasher.update(&now_secs.to_le_bytes());
    hasher.update(fragment.as_bytes());
    let id = hasher.finalize().to_hex()[..16].to_string();

    let local_msg = LocalMessage {
        id,
        from: format!("{}#scheduler", ctx.our_did),
        to: format!("{}#{}", ctx.our_did, fragment),
        created_at: now_secs,
        exp: 0,
        reply_to: None,
        message_type: MESSAGE_TYPE_RPC.to_string(),
        content_type: CONTENT_TYPE_TERM.to_string(),
        content: content.to_vec(),
    };

    let cast_input = CastInput {
        msg: crate::entity::PluginMsg::from(&local_msg),
    };

    let result = plugin.on_message(&cast_input).await;

    let result = match result {
        Ok(r) => r,
        Err(e) => {
            warn!(fragment = %fragment, error = %e, "scheduled dispatch error");
            return;
        }
    };

    // Persist state if changed.
    if let Some(state_bytes) = result.pending_state {
        match crate::kubo::ipfs_add_bytes_unpinned(&ctx.kubo_rpc_url, state_bytes.clone()).await {
            Ok(cid) => {
                let writer = ctx.manifest_writer.read().await.clone();
                let Some(writer) = writer else {
                    warn!(
                        fragment = %fragment,
                        cid = %cid,
                        "scheduled dispatch: state saved to IPFS but manifest writer is not ready"
                    );
                    return;
                };
                match writer.set_entity_state(fragment, &cid).await {
                    Ok(root_cid) => {
                        plugin.mark_saved(state_bytes);
                        trace!(fragment = %fragment, cid = %cid, %root_cid, "scheduled dispatch: entity state persisted");
                    }
                    Err(e) => warn!(
                        fragment = %fragment,
                        cid = %cid,
                        error = %e,
                        "scheduled dispatch: failed to update manifest with entity state"
                    ),
                }
            }
            Err(e) => warn!(
                fragment = %fragment,
                error = %e,
                "scheduled dispatch: state save failed"
            ),
        }
    }
}

#[cfg(test)]
mod duration_tests {
    use super::parse_duration;

    #[test]
    fn single_units() {
        assert_eq!(parse_duration("90s").unwrap().as_secs(), 90);
        assert_eq!(parse_duration("5m").unwrap().as_secs(), 300);
        assert_eq!(parse_duration("2h").unwrap().as_secs(), 7_200);
        assert_eq!(parse_duration("1d").unwrap().as_secs(), 86_400);
    }

    #[test]
    fn combined_units() {
        assert_eq!(parse_duration("1h30m").unwrap().as_secs(), 5_400);
        assert_eq!(parse_duration("2d12h").unwrap().as_secs(), 216_000);
    }

    #[test]
    fn rejects_zero() {
        assert!(parse_duration("0s").is_err());
    }

    #[test]
    fn rejects_unknown_unit() {
        assert!(parse_duration("5x").is_err());
    }

    #[test]
    fn rejects_trailing_number() {
        assert!(parse_duration("5").is_err());
    }
}
