//! # trios-igla-race-hack — GOLD III Ring
//!
//! CLI surface and coordination layer for IGLA RACE.  Contains
//! hive automaton for multi-agent coordination, lesson generation from
//! pruned trials, and Neon database integration for telemetry.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │              HACK LAYER                  │
//! │  (trios-igla-race-hack)                   │
//! ├─────────────────────────────────────────────┤
//! │ hive_automaton   │ Hive automaton (L13)            │
//! │ lessons          │ Failure memory (lesson gen)      │
//! │ neon             │ Neon DB client (STUB)          │
//! └─────────────────────────────────────────────┘
//!         │               │                  │
//!         ▼               │                  │
//! ┌──────────────┐      PIPELINE LAYER      └──────────────┘
//! │  trial_runner   │ (trios-igla-race-pipeline)          │
//! │  asha         │ (trios-algorithm-arena)           │
//! │  sampler       │ (trios-algorithm-arena)           │
//! │  worker         │ (trios-algorithm-arena)           │
//! └──────────────┘
//! ````
//!
//! ## Module Organization
//!
//! - `hive_automaton` — Cellular automaton for multi-agent coordination
//! - `lessons` — Lesson generation from pruned trials
//! - `neon` — Neon database integration
//! - `cli` — Command-line interface
//! - `api` — HTTP API server
//! - `status` — Race status queries
//! - `dashboard` — Live dashboard with event streaming
//!
//! ## External Dependencies
//!
//! - `trios-igla-race-pipeline` — Core pipeline logic
//! - `trios-algorithm-arena` — Validation, invariants, victory checking

pub mod hive_automaton;
pub mod lessons;
pub mod neon;
pub mod cli;
pub mod api;
pub mod status;
pub mod dashboard;

// Re-exports from dependencies for convenience
pub use hive_automaton::{
    HiveAutomaton, State, AgentAction, HaltCause, World,
    Lane, SCHEMA_VERSION, VICTORY_SEED_TARGET, BPB_VICTORY_TARGET, LANE_COUNT,
};
pub use lessons::{
    LessonType, Outcome, TrialConfig, RungData, generate_lesson, store_lesson, get_top_lessons,
};

/// HACK version — increments on breaking API changes
pub const HACK_VERSION: &str = "0.1.0";
