//! IGLA RACE — CLI entry point.
//!
//! Main binary for IGLA RACE system.

use anyhow::Result;
use tracing_subscriber;

mod cli;
mod status;
mod dashboard;
mod api;
mod hive_automaton;
mod lessons;
mod neon;

pub use cli::{run_cli, Cli};
pub use status::{RaceStatus, QueryStatus};
pub use dashboard::{Dashboard, DashboardEvent};
pub use api::{ApiServer, ApiConfig};
pub use hive_automaton::{HiveAutomaton, World};
pub use lessons::{generate_lesson, store_lesson, get_top_lessons};
pub use neon::{NeonDb, spawn_heartbeat};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    // Run CLI
    run_cli().await?;

    Ok(())
}
