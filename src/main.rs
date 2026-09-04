mod acl;
mod behaviour;
mod boot;
mod bootstrap;
mod crud;
mod dispatch;
mod doccache;
mod entity;
mod eventloop;
mod i18n;
mod ipfs;
mod kubo;
mod manifest;
mod plugin;
mod republish;
mod routing;
mod schedule;
mod scheduler_actor;
mod startup;
mod status;

#[cfg(test)]
mod testkubo;

use anyhow::Result;
use boot::{Boot, Cli};

/// Startup runs as a fixed sequence of phases on [`Boot`] (see `boot.rs`),
/// mirroring the `EventLoopState` pattern already used by `eventloop::run`.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_and_sanitise();

    if cli.ma.gen_headless_config {
        boot::generate_and_persist_headless_config(&cli.ma)?;
        return Ok(());
    }

    let mut boot = Boot::new(cli)?;
    if boot.run_cli_subcommands().await?.is_break() {
        return Ok(());
    }
    boot.setup_endpoint().await?;
    boot.build_identity_and_resolve_root().await?;
    boot.load_i18n_and_ipfs_state().await?;
    boot.load_entities().await?;
    boot.load_acl_owners_and_signing().await?;
    // _sched must stay alive for the process lifetime — dropping it stops
    // the native scheduler's background task.
    let (run_args, _sched) = boot.finalize().await?;

    eventloop::run(run_args).await
}
