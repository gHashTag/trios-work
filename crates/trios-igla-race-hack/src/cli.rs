//! CLI — Command-line interface for IGLA RACE.
//!
//! Provides subcommands:
//! - `start` — Start a new race with worker pool
//! - `status` — Query race status
//! - `best` — Show best BPB across all races
//! - `dashboard` — Launch live dashboard with event streaming

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, error};

use trios_algorithm_arena::{
    validate_config, TrialConfig,
    WorkerPool, PoolConfig, WorkerResult, TrialStatus,
};

use crate::{
    api::{ApiServer, ApiConfig},
    status::{RaceStatus, QueryStatus},
    dashboard::{Dashboard, DashboardEvent},
    hive_automaton::{HiveAutomaton, World},
};

/// Run a new race.
pub async fn run_race(pool: PoolConfig) -> Result<()> {
    info!("Starting IGLA RACE with {} workers, {} trials per worker",
            pool.workers, pool.trials_per_worker);

    // Preflight check: validate static config
    let probe = TrialConfig {
        lr: 0.004,
        d_model: 384,
        bpb_prune_threshold: 3.5,
        warmup_blind_steps: 4000,
        use_gf16: false,
        nca_grid: 81,
        nca_k_states: 9,
        grad_mode: crate::trios_algorithm_arena::GradientMode::RealMSE,
        current_step: 0,
        last_bpb: f64::MAX,
    };
    validate_config(&probe)?;

    // TODO: Actually run worker pool (needs async WorkerPool)
    info!("Race configuration validated (trials would run in future release)");
    Ok(())
}

/// Start subcommand.
#[derive(Parser, Debug)]
pub struct Start {
    /// Number of OS threads to spawn (≥ 1).
    #[arg(short = 'w', long = "workers", default_value_t = "4")]
    pub workers: u32,

    /// Trials each worker runs before joining.
    #[arg(short = 't', long = "trials", default_value_t = "8")]
    pub trials_per_worker: u32,

    /// Path of CSV telemetry sink.
    #[arg(short = 'o', long = "output", value_name_t = "telemetry_path")]
    pub telemetry_path: Option<String>,

    /// Base seed; worker `w` uses `base_seed.wrapping_add(w as u64)`.
    #[arg(short = 's', long = "seed", default_value_t = "42")]
    pub base_seed: u64,

    /// `d_model` value used by every trial.
    #[arg(short = 'd', long = "d-model", default_value_t = "384")]
    pub d_model: usize,

    /// Whether trials run in GF16 domain.
    #[arg(short = 'g', long = "gf16", default_value_t = "false")]
    pub use_gf16: bool,
}

/// Status subcommand.
#[derive(Subcommand, Debug)]
pub enum Status {
    Start,
}

impl Status {
    async fn execute(&self) -> Result<()> {
        match self {
            Status::Start => {
                let pool = PoolConfig {
                    workers: 4,
                    trials_per_worker: 8,
                    telemetry_path: Some("data/igla_trials.csv".to_string()),
                    base_seed: 42,
                    d_model: 384,
                    use_gf16: false,
                };
                run_race(pool).await?;
            }
            _ => Ok(()),
        }
    }
}

/// Best subcommand.
#[derive(Subcommand, Debug)]
pub struct Best {
    /// Show top N results.
    #[arg(short = 'n', long = "count", default_value_t = "5")]
    pub count: u32,
}

/// Dashboard subcommand.
#[derive(Subcommand, Debug)]
pub struct Dashboard {
    /// Launch live dashboard with event streaming.
    #[arg(long = "port", default_value_t = "8080")]
    pub port: u16,
}

/// Main CLI parser.
#[derive(Parser, Debug)]
#[command(name = "trios-igla-race", about = "IGLA RACE - Test-Time Training Pipeline")]
struct Cli {
    #[command(subcommand)]
    Start(Start),
    Status(Status),
    Best(Best),
    Dashboard(Dashboard),
}

/// Run CLI with given args.
pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Start(args) => {
            let pool = PoolConfig {
                workers: args.workers,
                trials_per_worker: args.trials_per_worker,
                telemetry_path: args.telemetry_path,
                base_seed: args.base_seed,
                d_model: args.d_model,
                use_gf16: args.use_gf16,
            };
            run_race(pool).await?;
        }
        _ => Ok(()),
    }
}
