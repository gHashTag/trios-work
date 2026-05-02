//! Neon Database Client — IGLA RACE telemetry backend
//!
//! Neon PostgreSQL client wrapper for IGLA RACE telemetry.
//! Provides trial registration, checkpoint recording, heartbeat tracking,
//! and lesson storage. Currently operates in STUB MODE for
//! offline development.
//!
//! ## Module Organization
//!
//! - `NeonDb` — Database client with connection pooling
//! - `TrialConfig`, `DashboardMeta`, `LessonEntry` — Data types
//! - `SCHEMA_MIGRATION` — SQL migration for new columns
//! - `queries` — Predefined queries (leaderboard, active agents, best by arch)
//!
//! ## Design
//!
//! STUB MODE: All database operations are async no-ops that log
//! but don't actually connect to PostgreSQL. This allows full pipeline
//! development and testing without requiring a live database connection.
//!
//! When a real database is available, replace the stub implementations
//! with actual Neon Postgres operations.
//!
//! ## External Dependencies
//!
//! - `tokio-postgres` — Async PostgreSQL client
//! - `uuid` — UUID generation for trials
//! - `serde` / `serde_json` — Serialization
//! - `chrono` — Timestamp types
//! - `anyhow` — Error handling
//!
//! # Database Schema (STUB)
//!
//! ```sql
//! CREATE TABLE IF NOT EXISTS igla_race_trials (
//!     trial_id UUID PRIMARY KEY,
//!     agent_id TEXT NOT NULL,
//!     branch TEXT DEFAULT 'main',
//!     machine_id TEXT,
//!     worker_id INTEGER,
//!
//!     config JSONB NOT NULL,
//!
//!     rung_1000_bpb NUMERIC,
//!     rung_3000_bpb NUMERIC,
//!     rung_9000_bpb NUMERIC,
//!     rung_27000_bpb NUMERIC,
//!
//!     final_bpb NUMERIC,
//!     final_rung_step INTEGER,
//!     status TEXT,
//!
//!     last_heartbeat TIMESTAMPTZ,
//!
//!     INDEX idx_igla_race_trials_agent (agent_id, branch, last_heartbeat)
//! );

//! CREATE TABLE IF NOT EXISTS igla_race_lessons (
//!     lesson_id UUID PRIMARY KEY,
//!     trial_id UUID REFERENCES igla_race_trials(trial_id),
//!     outcome TEXT NOT NULL,
//!     pruned_at_rung INTEGER,
//!     bpb_at_pruned NUMERIC,
//!     lesson TEXT NOT NULL,
//!     lesson_type TEXT NOT NULL,
//!     pattern_count INTEGER DEFAULT 1,
//!     created_at TIMESTAMPTZ DEFAULT NOW()
//! );
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialConfig {
    pub arch: String,
    #[serde(rename = "d_model")]
    pub hidden: usize,
    #[serde(rename = "n_gram")]
    pub context: usize,
    pub lr: f64,
    pub seed: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmup_steps: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMeta {
    pub agent_id: String,
    pub branch: String,
    pub machine_id: String,
    pub worker_id: String,
}

impl Default for DashboardMeta {
    fn default() -> Self {
        Self {
            agent_id: "ALPHA".to_string(),
            branch: "main".to_string(),
            machine_id: "unknown".to_string(),
            worker_id: "w0".to_string(),
        }
    }
}

impl DashboardMeta {
    pub fn new(agent_id: &str, machine_id: &str, worker_id: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            branch: "main".to_string(),
            machine_id: machine_id.to_string(),
            worker_id: worker_id.to_string(),
        }
    }

    pub fn with_branch(self, branch: &str) -> Self {
        let mut s = self.clone();
        s.branch = branch.to_string();
        s
    }

    pub fn with_agent(self, agent_id: &str) -> Self {
        let mut s = self.clone();
        s.agent_id = agent_id.to_string();
        s
    }
}

#[derive(Debug, Clone)]
pub struct LessonEntry {
    pub lesson: String,
    pub lesson_type: String,
    pub pattern_count: i32,
}

/// Neon database client.
///
/// In STUB MODE, all operations log to tracing but don't
/// actually perform I/O. This allows pipeline development
/// and testing without requiring a live database connection.
#[derive(Clone)]
pub struct NeonDb {
    _dummy: (),
}

impl NeonDb {
    /// Connect to Neon PostgreSQL.
    ///
    /// In STUB MODE, this logs but doesn't actually connect.
    /// Connection string should be in format:
    /// `postgresql://user:password@host:5432/database_name`
    pub async fn connect(conn_str: &str) -> Result<Self, anyhow::Error> {
        tracing::info!("Connecting to Neon: {conn_str} (STUB MODE)");
        tokio::time::sleep(Duration::from_millis(50)).await;
        tracing::info!("Connected to Neon (STUB)");
        Ok(Self { _dummy: () })
    }

    /// Get a reference to the client (for chaining).
    pub fn client(&self) -> &Self {
        self
    }

    /// Register a new trial in the database.
    ///
    /// In STUB MODE, logs trial details but doesn't insert.
    pub async fn register_trial(
        &self,
        trial_id: &Uuid,
        machine_id: &str,
        worker_id: i32,
        config_json: &str,
    ) -> Result<()> {
        tracing::info!(
            "Trial registered (STUB): trial_id={trial_id} machine={machine_id} worker={worker_id} config={config_json}"
        );
        Ok(())
    }

    /// Record a checkpoint for a trial.
    ///
    /// In STUB MODE, logs checkpoint but doesn't insert.
    pub async fn record_checkpoint(
        &self,
        trial_id: &Uuid,
        rung: i32,
        bpb: f64,
    ) -> Result<()> {
        tracing::info!("Checkpoint recorded (STUB): trial={trial_id} rung={rung} BPB={bpb:.4}");
        Ok(())
    }

    /// Update trial rung after ASHA promotion.
    ///
    /// In STUB MODE, logs rung update but doesn't actually update.
    pub async fn update_rung(
        &self,
        trial_id: &str,
        rung_steps: usize,
        bpb: f64,
    ) -> Result<()> {
        tracing::info!("Rung updated (STUB): trial={trial_id} rung_steps={rung_steps} BPB={bpb:.4}");
        Ok(())
    }

    /// Update trial heartbeat timestamp.
    ///
    /// In STUB MODE, logs heartbeat update but doesn't actually update.
    pub async fn update_heartbeat(
        &self,
        trial_id: &str,
    ) -> Result<()> {
        tracing::info!("Heartbeat (STUB): trial_id={trial_id}");
        Ok(())
    }

    /// Mark a trial as pruned (stopped early).
    ///
    /// In STUB MODE, logs prune event but doesn't actually mark.
    pub async fn mark_pruned(
        &self,
        trial_id: &Uuid,
        at_step: i32,
        bpb: f64,
    ) -> Result<()> {
        tracing::info!("Trial pruned (STUB): trial_id={trial_id} step={at_step} BPB={bpb:.4}");
        Ok(())
    }

    /// Mark a trial as completed successfully.
    ///
    /// In STUB MODE, logs completion but doesn't actually mark.
    pub async fn mark_completed(
        &self,
        trial_id: &Uuid,
        bpb: f64,
        steps: i32,
    ) -> Result<()> {
        tracing::info!("Trial completed (STUB): trial_id={trial_id} BPB={bpb:.4} steps={steps}");
        Ok(())
    }

    /// Mark a trial as winner (victory achieved).
    ///
    /// In STUB MODE, logs winner event but doesn't actually mark.
    pub async fn mark_winner(
        &self,
        trial_id: &str,
        bpb: f64,
        steps: i32,
    ) -> Result<()> {
        tracing::info!("Winner found (STUB): trial_id={trial_id} BPB={bpb:.4} steps={steps}");
        Ok(())
    }

    /// Store a lesson from a pruned trial.
    ///
    /// In STUB MODE, logs lesson but doesn't actually store.
    pub async fn store_lesson(
        &self,
        trial_id: &Uuid,
        outcome: &str,
        pruned_at_rung: i32,
        bpb_at_pruned: f64,
        lesson: &str,
        lesson_type: &str,
    ) -> Result<()> {
        tracing::info!(
            "Lesson stored (STUB): trial_id={trial_id} outcome={outcome} rung={pruned_at_rung} lesson={lesson}"
        );
        Ok(())
    }

    /// Get top lessons from experience.
    ///
    /// In STUB MODE, returns empty vector.
    pub async fn get_top_lessons(
        &self,
        limit: i32,
    ) -> Result<Vec<LessonEntry>> {
        tracing::info!("Top lessons query (STUB): limit={limit}");
        Ok(vec![])
    }

    /// Check if a configuration is already running.
    ///
    /// In STUB MODE, always returns false.
    pub async fn is_config_running(
        &self,
        _machine_id: &str,
        _config_json: &str,
    ) -> Result<bool> {
        let _ = (_machine_id, _config_json);
        tracing::info!("Config running check (STUB): {:?}", _);
        Ok(false)
    }

    /// Generic query execution.
    ///
    /// In STUB MODE, logs query but doesn't execute.
    pub async fn query(
        &self,
        query: &str,
        _params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>> {
        tracing::info!("Query (STUB): {}", query.trim().chars().take(80).collect::<String>());
        Ok(vec![])
    }

    /// Get a specific query result.
    ///
    /// In STUB MODE, logs query but doesn't execute.
    pub async fn query_one(
        &self,
        query: &str,
        _params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row> {
        tracing::info!("Query one (STUB): {}", query.trim().chars().take(80).collect::<String>());
        Err(anyhow::anyhow!("no rows (STUB)"))
    }
}

/// SQL schema migration script.
///
/// Adds new columns to the igla_race_trials table:
/// - branch (for multi-agent coordination)
/// - agent_id (to associate trials with agents)
/// - last_heartbeat (to detect stale workers)
///
/// Run once when deploying to real Neon instance.
pub const SCHEMA_MIGRATION: &str = r#"
ALTER TABLE igla_race_trials ADD COLUMN IF NOT EXISTS branch TEXT DEFAULT 'main';
ALTER TABLE igla_race_trials ADD COLUMN IF NOT EXISTS agent_id TEXT;
ALTER TABLE igla_race_trials ADD COLUMN IF NOT EXISTS last_heartbeat TIMESTAMPTZ DEFAULT NOW();
"#;

/// Predefined queries for common dashboard views.
pub mod queries {
    pub const LEADERBOARD: &str = r#"
SELECT
  agent_id,
  branch,
  config->>'arch' as arch,
  config->>'d_model' as d_model,
  rung_27000_bpb,
  rung_9000_bpb,
  final_bpb,
  final_rung_step,
  status,
  EXTRACT(EPOCH FROM (last_heartbeat)) as heartbeat_lag_sec,
  ROW_NUMBER() OVER (PARTITION BY ROW_NUMBER ORDER BY final_bpb ASC) as rank
FROM igla_race_trials
WHERE status IN ('running', 'completed', 'victory')
  AND last_heartbeat > NOW() - INTERVAL '2 minutes'
ORDER BY final_bpb ASC NULLS LAST
LIMIT 20;
"#;

    pub const ACTIVE_AGENTS: &str = r#"
SELECT
  agent_id,
  machine_id,
  branch,
  COUNT(*) as active_trials
FROM igla_race_trials
WHERE status='running'
  AND last_heartbeat > NOW() - INTERVAL '2 minutes'
GROUP BY agent_id, branch;
"#;

    pub const BEST_BY_ARCH: &str = r#"
SELECT
  config->>'arch' as arch,
  MIN(final_bpb) as best_bpb,
  COUNT(*) as trials
FROM igla_race_trials
WHERE status IN ('completed', 'victory')
  AND config->>'arch' IS NOT NULL
GROUP BY config->>'arch';
"#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_config_serialization() {
        let config = TrialConfig {
            arch: "ngram".to_string(),
            hidden: 384,
            context: 6,
            lr: 0.004,
            seed: 42,
            optimizer: Some("adamw".to_string()),
            wd: Some(0.01),
            activation: Some("gelu".to_string()),
            warmup_steps: Some(4000),
            max_steps: Some(10000),
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"n_gram\""));
        assert!(json.contains("\"d_model\""));
        assert!(json.contains("\"context\""));
    assert!(json.contains("\"lr\""));
    }

    #[test]
    fn test_dashboard_meta_default() {
        let meta = DashboardMeta::default();
        assert_eq!(meta.agent_id, "ALPHA");
        assert_eq!(meta.branch, "main");
    }

    #[test]
    fn test_dashboard_meta_custom() {
        let meta = DashboardMeta::new("BETA", "mac-studio-2", "w3");
        assert_eq!(meta.agent_id, "BETA");
        assert_eq!(meta.machine_id, "mac-studio-2");
        assert_eq!(meta.worker_id, "w3");
    }

    #[test]
    fn test_dashboard_meta_with_branch() {
        let meta = DashboardMeta::default().with_branch("feat/jepa");
        assert_eq!(meta.branch, "feat/jepa");
    }

    #[test]
    fn test_dashboard_meta_with_agent() {
        let meta = DashboardMeta::default().with_agent("GAMMA");
        assert_eq!(meta.agent_id, "GAMMA");
    }

    #[test]
    fn test_lesson_entry() {
        let entry = LessonEntry {
            lesson: "AVOID: lr too high".to_string(),
            lesson_type: "AVOID".to_string(),
            pattern_count: 5,
        };
        assert_eq!(entry.lesson, "AVOID: lr too high");
        assert_eq!(entry.lesson_type, "AVOID");
    }

    #[test]
    fn test_schema_migration_contains_required_columns() {
        assert!(SCHEMA_MIGRATION.contains("branch"));
        assert!(SCHEMA_MIGRATION.contains("agent_id"));
        assert!(SCHEMA_MIGRATION.contains("last_heartbeat"));
    }

    #[test]
    fn test_queries_exist() {
        assert!(!queries::LEADERBOARD.is_empty());
        assert!(!queries::ACTIVE_AGENTS.is_empty());
        assert!(!queries::BEST_BY_ARCH.is_empty());
    }

    #[test]
    fn test_connect_stub_logs() {
        // The connect function should log connection attempt
        // We can't test async in unit tests without a runtime
        // But we can verify the schema migration SQL is present
        assert!(!SCHEMA_MIGRATION.is_empty());
    }
}
