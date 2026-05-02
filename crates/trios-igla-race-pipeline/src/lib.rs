//! # trios-igla-race-pipeline — E2E Test-Time Training Pipeline
//!
//! GOLD I ring — core pipeline logic for IGLA RACE. This crate
//! contains trial execution engine, ASHA hyperparameter optimization,
//! sampling utilities, and worker pool orchestration.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                    PIPELINE LAYER                    │
//! │  (trios-igla-race-pipeline)                     │
//! ├─────────────────────────────────────────────────────┤
//! │ trial_runner       │ Execute single trial (GPU path)    │
//! │ asha_scheduler     │ Successive Halving algorithm        │
//! │ lr_sampler        │ φ-band learning rate sampling      │
//! │ worker_pool       │ Parallel worker orchestration     │
//! │ telemetry         │ CSV/Neon event streaming       │
//! └─────────────────────────────────────────────────────┘
//!         │                        │
//!         ▼                        ▼
//! ┌──────────────┐    ┌───────────────────┐
//! │   ARENA     │    │     HACK         │
//! │ validation   │◄───│   CLI surface     │
//! └──────────────┘    └───────────────────┘
//! ```
//!
//! ## Module Organization
//!
//! - `trial` — Core trial execution logic
//! - `asha` — Successive Halving algorithm implementation
//! - `sampler` — φ-band learning rate sampling
//! - `worker` — Worker pool and trial distribution
//! - `telemetry` — Event streaming to CSV/Neon
//! - `race` — Worker pool with parallel execution
//!
//! ## External Dependencies
//!
//! - `trios-algorithm-arena` — Validation, invariants, victory checking

pub mod trial;
pub mod asha;
pub mod sampler;
pub mod worker;
pub mod telemetry;
pub mod race;

// Re-exports for convenience
pub use trial::{TrialConfig, TrialResult, TrialError};
pub use asha::{AshaConfig, AshaRung, run_asha};
pub use sampler::{LrSampler, LrSampleError};
pub use worker::{WorkerPool, WorkerConfig, WorkerResult};
pub use telemetry::{TelemetrySink, TelemetryEvent};

/// Pipeline version — increments on breaking API changes
pub const PIPELINE_VERSION: &str = "0.1.0";
