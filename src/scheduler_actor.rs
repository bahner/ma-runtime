//! Native `#scheduler` entity — compiled-in schedule-registration actor.
//!
//! `#scheduler` is a system entity whose implementation lives here in Rust
//! rather than in a Wasm module. It is loaded from the manifest through the
//! native factory registry.
//!
//! ## Protocol
//!
//! Any entity on this runtime can send a CBOR array to
//! `did:ma:<our_did>#scheduler` to register a scheduled dispatch:
//!
//! ```text
//! [name, ":cron",     spec_str,     verb_or_array, extra_args…]
//! [name, ":interval", duration_str, verb_or_array, extra_args…]
//! [name, ":at",       timestamp_ms, verb_or_array, extra_args…]
//! [name, ":random",   max_secs_int, verb_or_array, extra_args…]
//! ```
//!
//! Schedules are caller-owned. The dispatch target is always `msg.from`
//! (the registering entity). The scheduler keys jobs by
//! `<msg.from>-<name>`, so re-registering the same `name` from the same sender
//! replaces the previous job deterministically.
//!
//! ## ACL
//!
//! `#scheduler` uses the `"scheduler"` ACL entry from the root manifest.
//! A typical ACL grants `:help` to everyone while keeping registration verbs
//! local-only, for example `"*": [":help"]` together with
//! `"#": [":cron", ":interval", ":at", ":random"]`.
//!
//! ## Kind
//!
//! Kind protocol: [`SCHEDULER_KIND`] (`/ma/scheduler/0.0.1`)
//! Evaluator: `native`
//! Dispatch surface: `on_message`

use std::sync::Arc;
use std::sync::PoisonError;
use std::{collections::HashMap, sync::Mutex};

use anyhow::{anyhow, Result};
use ciborium::Value as CborValue;
use tracing::{trace, warn};

use crate::entity::CastInput;
use crate::plugin::{DispatchResult, NativeActor, NativeFactory, NativeSignal};
use crate::schedule::{
    parse_duration, register_schedule, ActiveScheduleGuard, ScheduleRequest, SchedulerCtx,
};

/// Kind protocol ID for the native scheduler entity.
pub const SCHEDULER_KIND: &str = "/ma/scheduler/0.0.1";

/// Build the native actor for `#scheduler`.
///
/// The closure captures `sched` (the running [`tokio_cron_scheduler::JobScheduler`])
/// and `ctx` (the [`SchedulerCtx`] needed by [`register_schedule`]).
///
/// On each call it parses the incoming CBOR array, spawns an async task to
/// register the schedule, and returns `:ok` immediately (fire-and-forget).
/// Parse errors are returned as `Err` so the caller can send an error reply.
pub fn native_actor(
    sched: Arc<tokio_cron_scheduler::JobScheduler>,
    ctx: SchedulerCtx,
) -> NativeActor {
    let schedule_registry = ScheduleRegistry::new();
    let signal_schedule_registry = schedule_registry.clone();
    NativeActor::new(move |input: &CastInput| -> Result<DispatchResult> {
        let term: CborValue = ciborium::de::from_reader(input.msg.content.as_slice())
            .map_err(|e| anyhow!("scheduler: invalid CBOR in message: {e}"))?;

        if is_help_request(&term) {
            return help_dispatch_result();
        }

        let req = parse_schedule_request(term, &input.msg.from)?;
        tokio::spawn(process_schedule_registration(
            Arc::clone(&sched),
            ctx.clone(),
            schedule_registry.clone(),
            input.msg.from.clone(),
            req,
        ));

        let mut out = Vec::new();
        ciborium::ser::into_writer(&CborValue::Text(":ok".to_string()), &mut out)
            .map_err(|e| anyhow!("scheduler: CBOR encode :ok: {e}"))?;
        Ok(DispatchResult {
            output: out,
            pending_state: None,
            create_requests: vec![],
            delete_requests: vec![],
            behaviour_requests: vec![],
        })
    })
    .with_state_hooks(|| None, |_| {})
    .with_signal(move |signal| {
        let shutdown = matches!(&signal, NativeSignal::Shutdown);
        match signal {
            NativeSignal::SetState(_bytes) => {}
            NativeSignal::Init(_payload) => {}
            NativeSignal::Start | NativeSignal::Shutdown => {}
        }
        if shutdown {
            signal_schedule_registry.deactivate_all();
        }
        Ok(())
    })
}

/// Build the native factory used by manifest loading for `/ma/scheduler/0.0.1`.
pub fn native_factory(
    sched: Arc<tokio_cron_scheduler::JobScheduler>,
    ctx: SchedulerCtx,
) -> NativeFactory {
    Arc::new(move || native_actor(Arc::clone(&sched), ctx.clone()))
}

// ── Internal types ────────────────────────────────────────────────────────────

struct ParsedRequest {
    schedule_key: String,
    fragment: String,
    request: ScheduleRequest,
}

#[derive(Clone, Copy)]
struct TrackedJob {
    version: u64,
    job_id: Option<uuid::Uuid>,
}

type ScheduleJobs = Arc<Mutex<HashMap<String, TrackedJob>>>;

#[derive(Clone)]
struct ScheduleRegistry {
    jobs: ScheduleJobs,
}

struct RegistrationAttempt {
    version: u64,
    previous_job: Option<uuid::Uuid>,
}

enum RegistrationCommit {
    Active,
    Superseded,
}

impl ScheduleRegistry {
    fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn begin_registration(&self, schedule_key: &str) -> RegistrationAttempt {
        let mut map = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
        let previous = map.get(schedule_key).copied();
        let version = previous.map_or(1, |entry| entry.version.saturating_add(1));
        map.insert(
            schedule_key.to_string(),
            TrackedJob {
                version,
                job_id: None,
            },
        );
        drop(map);
        RegistrationAttempt {
            version,
            previous_job: previous.and_then(|entry| entry.job_id),
        }
    }

    fn active_guard(&self, schedule_key: String, version: u64) -> ActiveScheduleGuard {
        let jobs = Arc::clone(&self.jobs);
        Arc::new(move || {
            let map = jobs.lock().unwrap_or_else(PoisonError::into_inner);
            map.get(&schedule_key)
                .is_some_and(|entry| entry.version == version)
        })
    }

    fn commit_registration(
        &self,
        schedule_key: &str,
        version: u64,
        job_id: uuid::Uuid,
    ) -> RegistrationCommit {
        let mut map = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
        if map
            .get(schedule_key)
            .is_some_and(|entry| entry.version == version)
        {
            map.insert(
                schedule_key.to_string(),
                TrackedJob {
                    version,
                    job_id: Some(job_id),
                },
            );
            RegistrationCommit::Active
        } else {
            RegistrationCommit::Superseded
        }
    }

    fn rollback_registration(&self, schedule_key: &str, version: u64) {
        let mut map = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
        if map
            .get(schedule_key)
            .is_some_and(|entry| entry.version == version)
        {
            map.remove(schedule_key);
        }
    }

    fn deactivate_all(&self) {
        self.jobs
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();
    }
}

fn help_dispatch_result() -> Result<DispatchResult> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Text(scheduler_help_text().to_string()),
        &mut out,
    )
    .map_err(|e| anyhow!("scheduler: CBOR encode help: {e}"))?;
    Ok(DispatchResult {
        output: out,
        pending_state: None,
        create_requests: vec![],
        delete_requests: vec![],
        behaviour_requests: vec![],
    })
}

async fn process_schedule_registration(
    sched: Arc<tokio_cron_scheduler::JobScheduler>,
    ctx: SchedulerCtx,
    schedule_registry: ScheduleRegistry,
    from: String,
    req: ParsedRequest,
) {
    let attempt = schedule_registry.begin_registration(&req.schedule_key);

    trace!(
        from = %from,
        target = %req.fragment,
        schedule = %req.schedule_key,
        version = attempt.version,
        previous_job = ?attempt.previous_job,
        "scheduler: processing registration"
    );

    if let Some(job_id) = attempt.previous_job {
        if let Err(e) = sched.remove(&job_id).await {
            warn!(from = %from, schedule = %req.schedule_key, error = %e, "scheduler: failed to remove previous schedule job");
        } else {
            trace!(
                from = %from,
                schedule = %req.schedule_key,
                previous_job = %job_id,
                "scheduler: removed previous schedule job"
            );
        }
    }

    let active_guard = schedule_registry.active_guard(req.schedule_key.clone(), attempt.version);

    match register_schedule(
        &sched,
        ctx,
        req.fragment.clone(),
        Some(req.schedule_key.clone()),
        Some(active_guard),
        req.request,
    )
    .await
    {
        Ok(job_id) => {
            handle_registered_schedule_job(
                &sched,
                &schedule_registry,
                &from,
                &req.fragment,
                &req.schedule_key,
                attempt.version,
                job_id,
            )
            .await;
        }
        Err(e) => {
            schedule_registry.rollback_registration(&req.schedule_key, attempt.version);
            warn!(target = %req.fragment, from = %from, schedule = %req.schedule_key, error = %e, "scheduler: failed to register schedule");
        }
    }
}

async fn handle_registered_schedule_job(
    sched: &tokio_cron_scheduler::JobScheduler,
    schedule_registry: &ScheduleRegistry,
    from: &str,
    fragment: &str,
    schedule_key: &str,
    new_version: u64,
    job_id: uuid::Uuid,
) {
    match schedule_registry.commit_registration(schedule_key, new_version, job_id) {
        RegistrationCommit::Superseded => {
            if let Err(e) = sched.remove(&job_id).await {
                warn!(from = %from, schedule = %schedule_key, error = %e, "scheduler: failed to remove superseded schedule job");
            } else {
                trace!(
                    from = %from,
                    schedule = %schedule_key,
                    version = new_version,
                    job_id = %job_id,
                    "scheduler: removed superseded job registered by stale task"
                );
            }
        }
        RegistrationCommit::Active => {
            trace!(
                from = %from,
                target = %fragment,
                schedule = %schedule_key,
                version = new_version,
                job_id = %job_id,
                "scheduler: registered active schedule job"
            );
        }
    }
}

fn is_help_request(term: &CborValue) -> bool {
    match term {
        CborValue::Text(s) => s == ":help",
        CborValue::Array(items) if items.len() == 1 => {
            matches!(items.first(), Some(CborValue::Text(s)) if s == ":help")
        }
        _ => false,
    }
}

const fn scheduler_help_text() -> &'static str {
    "scheduler help\n\
format: [name, :type, spec, verb_or_array, extra_args...]\n\
types: :cron, :interval, :at, :random\n\
specs: :cron=\"sec min hour day month weekday\", :interval=\"30m\", :at=<unix_ms>, :random=<max_secs>\n\
ownership: target is always msg.from; same [msg.from + name] overwrites previous schedule"
}

// ── CBOR schedule-request parser ─────────────────────────────────────────────

/// Parse a CBOR schedule-registration array into a [`ScheduleRequest`].
///
/// Expected wire format:
/// ```text
/// [name, type_atom, spec, verb_or_array, extra_args…]
/// ```
/// where `type_atom` is one of `:cron`, `:interval`, `:at`, `:random`.
fn parse_schedule_request(term: CborValue, from: &str) -> Result<ParsedRequest> {
    let items = match term {
        CborValue::Array(a) => a,
        other => return Err(anyhow!("scheduler: expected CBOR array, got {other:?}")),
    };
    if items.len() < 4 {
        return Err(anyhow!(
            "scheduler: expected [name, type, spec, verb], got {} elements",
            items.len()
        ));
    }

    let schedule_name = match &items[0] {
        CborValue::Text(s) if !s.is_empty() => s.clone(),
        CborValue::Text(_) => return Err(anyhow!("scheduler: schedule name must not be empty")),
        _ => {
            return Err(anyhow!(
                "scheduler: first element must be schedule name text"
            ))
        }
    };

    let type_verb = match &items[1] {
        CborValue::Text(s) => s.clone(),
        _ => return Err(anyhow!("scheduler: second element must be text atom")),
    };

    // Ownership is caller-scoped: dispatch target is always the sender fragment.
    let fragment = from
        .split_once('#')
        .map(|(_, frag)| frag.to_string())
        .filter(|frag| !frag.is_empty())
        .ok_or_else(|| anyhow!("scheduler: sender must be DID-URL with fragment"))?;
    let schedule_key = format!("{from}-{schedule_name}");

    // Encode verb (4th element) + optional inline args (5th+) as CBOR content bytes.
    let content = encode_verb_content(&items)?;

    let request = match type_verb.as_str() {
        ":cron" => {
            let spec = match &items[2] {
                CborValue::Text(s) => s.clone(),
                _ => return Err(anyhow!("scheduler :cron: spec must be text")),
            };
            ScheduleRequest::Cron { spec, content }
        }
        ":interval" => {
            let dur_str = match &items[2] {
                CborValue::Text(s) => s.clone(),
                _ => return Err(anyhow!("scheduler :interval: duration must be text")),
            };
            let secs = parse_duration(&dur_str)
                .map_err(|e| anyhow!("scheduler :interval: {e}"))?
                .as_secs();
            ScheduleRequest::Interval { secs, content }
        }
        ":at" => {
            let ts = match &items[2] {
                CborValue::Integer(n) => i64::try_from(i128::from(*n))
                    .map_err(|_| anyhow!("scheduler :at: timestamp out of i64 range"))?,
                _ => return Err(anyhow!("scheduler :at: timestamp must be integer")),
            };
            ScheduleRequest::At {
                timestamp_ms: ts,
                content,
            }
        }
        ":random" => {
            let max_secs = match &items[2] {
                CborValue::Integer(n) => u64::try_from(i128::from(*n)).unwrap_or(60),
                _ => return Err(anyhow!("scheduler :random: max_secs must be integer")),
            };
            ScheduleRequest::Random { max_secs, content }
        }
        other => return Err(anyhow!("scheduler: unknown schedule type '{other}'")),
    };

    Ok(ParsedRequest {
        schedule_key,
        fragment,
        request,
    })
}

/// Encode the verb (4th element) and any extra args (5th+) as pre-encoded CBOR bytes.
fn encode_verb_content(items: &[CborValue]) -> Result<Vec<u8>> {
    let args = items.get(4..).unwrap_or(&[]);
    let value = match &items[3] {
        CborValue::Text(v) if args.is_empty() => CborValue::Text(v.clone()),
        CborValue::Text(v) => {
            let mut arr = vec![CborValue::Text(v.clone())];
            arr.extend_from_slice(args);
            CborValue::Array(arr)
        }
        arr @ CborValue::Array(_) => arr.clone(),
        _ => {
            return Err(anyhow!(
                "scheduler: verb (4th element) must be text atom or array"
            ))
        }
    };
    let mut out = Vec::new();
    ciborium::ser::into_writer(&value, &mut out)
        .map_err(|e| anyhow!("scheduler: CBOR encode verb: {e}"))?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{
        encode_verb_content, is_help_request, parse_schedule_request, scheduler_help_text,
        RegistrationCommit, ScheduleRegistry,
    };
    use crate::schedule::ScheduleRequest;
    use ciborium::Value as CborValue;

    fn text(s: &str) -> CborValue {
        CborValue::Text(s.to_string())
    }

    fn int(n: i64) -> CborValue {
        CborValue::Integer(n.into())
    }

    #[test]
    fn parses_cron() {
        let term = CborValue::Array(vec![
            text("tick"),
            text(":cron"),
            text("0 * * * * *"),
            text(":tick"),
        ]);
        let p = parse_schedule_request(term, "did:ma:abc#myentity").unwrap();
        assert_eq!(p.fragment, "myentity");
        assert_eq!(p.schedule_key, "did:ma:abc#myentity-tick");
        assert!(matches!(p.request, ScheduleRequest::Cron { spec, .. } if spec == "0 * * * * *"));
    }

    #[test]
    fn parses_interval_duration_to_seconds() {
        let term = CborValue::Array(vec![
            text("grow"),
            text(":interval"),
            text("30m"),
            text(":grow"),
        ]);
        let p = parse_schedule_request(term, "did:ma:abc#garden").unwrap();
        assert!(matches!(p.request, ScheduleRequest::Interval { secs, .. } if secs == 1_800));
    }

    #[test]
    fn parses_at_timestamp() {
        let term = CborValue::Array(vec![
            text("wake-once"),
            text(":at"),
            int(1_700_000_000_000),
            text(":wake"),
        ]);
        let p = parse_schedule_request(term, "did:ma:abc#e").unwrap();
        assert!(
            matches!(p.request, ScheduleRequest::At { timestamp_ms, .. } if timestamp_ms == 1_700_000_000_000)
        );
    }

    #[test]
    fn parses_random_max_secs() {
        let term = CborValue::Array(vec![
            text("scratch"),
            text(":random"),
            int(300),
            text(":scratch"),
        ]);
        let p = parse_schedule_request(term, "did:ma:abc#dog").unwrap();
        assert!(matches!(p.request, ScheduleRequest::Random { max_secs, .. } if max_secs == 300));
    }

    #[test]
    fn uses_sender_fragment_as_target() {
        let term = CborValue::Array(vec![
            text("tick"),
            text(":cron"),
            text("* * * * * *"),
            text(":tick"),
        ]);
        let p = parse_schedule_request(term, "did:ma:abc#myentity").unwrap();
        assert_eq!(p.fragment, "myentity");
    }

    #[test]
    fn rejects_too_few_elements() {
        let term = CborValue::Array(vec![text("name"), text(":cron"), text("spec")]);
        assert!(parse_schedule_request(term, "did:ma:abc#entity").is_err());
    }

    #[test]
    fn rejects_unknown_type() {
        let term = CborValue::Array(vec![text("name"), text(":weekly"), text("x"), text(":v")]);
        assert!(parse_schedule_request(term, "did:ma:abc#entity").is_err());
    }

    #[test]
    fn rejects_sender_without_fragment() {
        let term = CborValue::Array(vec![text("name"), text(":cron"), text("x"), text(":v")]);
        assert!(parse_schedule_request(term, "did:ma:abc").is_err());
    }

    #[test]
    fn encode_verb_content_bare_verb_stays_atom() {
        let items = vec![text("n"), text(":x"), text("y"), text(":grow")];
        let content = encode_verb_content(&items).unwrap();
        let decoded: CborValue = ciborium::de::from_reader(content.as_slice()).unwrap();
        assert!(matches!(decoded, CborValue::Text(s) if s == ":grow"));
    }

    #[test]
    fn encode_verb_content_wraps_extra_args_in_array() {
        let items = vec![
            text("n"),
            text(":x"),
            text("y"),
            text(":grow"),
            text("a"),
            text("b"),
        ];
        let content = encode_verb_content(&items).unwrap();
        let decoded: CborValue = ciborium::de::from_reader(content.as_slice()).unwrap();
        assert!(matches!(decoded, CborValue::Array(a) if a.len() == 3));
    }

    #[test]
    fn random_overwrite_race_disables_stale_generation() {
        let key = "did:ma:abc#duck-quack".to_string();
        let registry = ScheduleRegistry::new();
        let first = registry.begin_registration(&key);
        let stale_guard = registry.active_guard(key.clone(), first.version);

        // Simulate overwrite race: before stale callback executes, version 2 wins.
        let second = registry.begin_registration(&key);
        assert_eq!(second.version, first.version + 1);

        // Stale generation must be blocked both at dispatch and at re-schedule check.
        assert!(!stale_guard());
        assert!(!stale_guard());

        let latest_guard = registry.active_guard(key, second.version);
        assert!(latest_guard());
    }

    #[test]
    fn deactivate_all_disables_registered_generations() {
        let key = "did:ma:abc#duck-quack".to_string();
        let registry = ScheduleRegistry::new();
        let registration = registry.begin_registration(&key);
        let guard = registry.active_guard(key, registration.version);

        assert!(guard());
        registry.deactivate_all();
        assert!(!guard());
    }

    #[test]
    fn commit_registration_rejects_stale_version() {
        let key = "did:ma:abc#duck-quack";
        let registry = ScheduleRegistry::new();
        let first = registry.begin_registration(key);
        let second = registry.begin_registration(key);

        let first_commit = registry.commit_registration(key, first.version, uuid::Uuid::nil());
        let second_commit = registry.commit_registration(key, second.version, uuid::Uuid::nil());

        assert!(matches!(first_commit, RegistrationCommit::Superseded));
        assert!(matches!(second_commit, RegistrationCommit::Active));
    }

    #[test]
    fn help_request_detection_supports_atom_and_singleton_array() {
        assert!(is_help_request(&text(":help")));
        assert!(is_help_request(&CborValue::Array(vec![text(":help")])));
        assert!(!is_help_request(&text(":cron")));
    }

    #[test]
    fn help_text_mentions_types_and_format() {
        let help = scheduler_help_text();
        assert!(help.contains("format:"));
        assert!(help.contains(":cron"));
        assert!(help.contains(":interval"));
        assert!(help.contains(":at"));
        assert!(help.contains(":random"));
    }
}
