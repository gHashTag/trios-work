//! # trios-algorithm-arena — GOLD II Ring
//!
//! Validator and specification wrapper for IGLA RACE.  Contains all
//! Coq-proven invariants, ASHA rung schedule (Trinity-anchored), and
//! victory checking (Welch t-test).
//!
//! ## Module Organization
//!
//! - `invariants` — IGLA INV-001..005 constants and validation gates
//! - `rungs` — ASHA rung schedule with Trinity base (INV-12)
//! - `victory` — Welch t-test victory detection (INV-7)
//! - `ema` — φ-anchored exponential moving average tracker
//! - `attn` — QK head shape guard for attention layers
//!
//! ## External Dependencies
//!
//! Uses only workspace dependencies (no external crates).
//!
//! # Constants
//!
//! All φ-anchored constants are defined in `invariants.rs`.
//! - PHI (golden ratio ≈ 1.618)
//! - PHI_INV = 1/φ ≈ 0.618
//! - PHI_SQ = φ² ≈ 2.618
//! - TRINITY_BASE = 3 (Trinity identity)
//!
//! # Invariants (INV-001..005)
//!
//! INV-1: φ-safe learning rate band [α_φ/φ³, φ⁻⁶/2]
//! INV-2: ASHA prune threshold = 3.5 (φ² + φ⁻² + φ⁻⁴)
//! INV-3: GF16 safe domain d_model ≥ 256
//! INV-4: NCA entropy band [φ, φ²] / K = 9
//! INV-5: GF16 Lucas closure consistency

pub mod invariants;
pub mod rungs;
pub mod victory;
pub mod ema;
pub mod attn;

// Re-exports for convenience
pub use invariants::{
    PHI, PHI_INV, PHI_SQ, PHI_INV_SQ, PHI_INV_6,
    INV1_LR_SAFE_LO, INV1_LR_SAFE_HI, INV1_CHAMPION_LR, INV1_SMOOTHNESS_L,
    INV2_BPB_PRUNE_THRESHOLD, INV2_WARMUP_BLIND_STEPS,
    INV3_D_MODEL_MIN, INV3_ERROR_BOUND,
    INV4_ENTROPY_CERTIFIED_LO, INV4_ENTROPY_CERTIFIED_HI,
    INV4_ENTROPY_EMPIRICAL_LO, INV4_ENTROPY_EMPIRICAL_HI,
    INV4_NCA_GRID, INV4_NCA_K_STATES,
    INV5_GF16_BITS, INV5_GF16_ELEMENTS,
    LUCAS_0, LUCAS_1, LUCAS_2, LUCAS_3, LUCAS_4,
    TrialConfig, InvTrialConfig, GradientMode, InvError,
    validate_config, enforce_all_invariants,
};

pub use rungs::{
    TRINITY_BASE, RUNG_UNIT, RUNG_UNIT,
    Rung, ALL, MAX_RUNG_EXP, RUNG_COUNT,
    iter_rungs, from_exp, from_step, check_inv12_rung_valid, check_inv12_rung_valid_usize,
};

pub use victory::{
    BPB_VICTORY_TARGET, VICTORY_SEED_TARGET,
    SeedResult, VictoryReport, VictoryError,
    check_victory, is_victory,
};

pub use ema::{EmaTracker, EmaError, ALPHA_PHI_INV_3, ALPHA_MIN_EXCLUSIVE, ALPHA_MAX_INCLUSIVE};

pub use attn::{QkHead, QkHeadError, PHI_4, HEAD_DIM_PHI_FLOOR, NUM_HEADS_MAX};

/// Arena version — increments on breaking API changes
pub const ARENA_VERSION: &str = "0.1.0";
