//! Status — Race status queries.
//!
//! Provides `QueryStatus` results for race status inquiries.
//! Used by CLI `status` subcommand.
//!
//! ## Module Organization
//!
//! - `QueryStatus` — Status enumeration for queries
//! - `RaceStatus` — Running/completed/pruned trials
//!
//! ## External Dependencies
//!
//! - `trios-algorithm-arena` — For validation logic

pub mod pool;

use anyhow::Result;
use crate::trios_algorithm_arena::{TrialConfig, WorkerPool, PoolConfig};

/// Query status enumeration.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryStatus {
    /// Query was successful
    Ok,
    /// No trials found matching criteria
    Empty,
    /// Database error occurred
    DbError(String),
}

/// Race status for a trial.
#[derive(Debug, Clone)]
pub struct RaceStatus {
    /// Trial ID
    pub trial_id: String,
    /// Worker ID
    pub worker_id: u32,
    /// Learning rate
    pub lr: f64,
    /// Model dimension
    pub d_model: usize,
    /// Current rung step
    pub rung_step: u32,
    /// BPB at last checkpoint
    pub bpb: f64,
    /// Trial status
    pub status: String,
}

/// Best trial result.
#[derive(Debug, Clone)]
pub struct BestResult {
    /// Trial ID
    pub trial_id: String,
    /// BPB
    pub bpb: f64,
    /// Seed
    pub seed: u64,
}

/// Query race status.
pub fn query_status(
    limit: usize,
    min_bpb: Option<f64>,
) -> Result<Vec<RaceStatus>, QueryStatus> {
    // TODO: Implement actual Neon query
    // For now return mock data
    let mut results = Vec::new();

    if let Some(min) = min_bpb {
        results.push(RaceStatus {
            trial_id: "mock-001".to_string(),
            worker_id: 1,
            lr: 0.004,
            d_model: 384,
            rung_step: 27000,
            bpb: 1.5,
            status: "completed".to_string(),
        });
    }

    if results.is_empty() {
        Ok((results, QueryStatus::Empty))
    } else {
        Ok((results, QueryStatus::Ok))
    }
}

/// Query best trial.
pub fn query_best(
    count: usize,
) -> Result<Vec<BestResult>, QueryStatus> {
    // TODO: Implement actual Neon query
    let mut results = Vec::new();

    for i in 1..=count {
        results.push(BestResult {
            trial_id: format!("mock-best-{:03}", i),
            bpb: 1.5 - (i as f64) * 0.01,
            seed: (42 + i) as u64,
        });
    }

    if results.is_empty() {
        Ok((results, QueryStatus::Empty))
    } else {
        Ok((results, QueryStatus::Ok))
    }
}
