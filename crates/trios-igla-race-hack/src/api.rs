//! API — HTTP API surface.
//!
//! Provides REST API for external systems to query and control
//! IGLA RACE. Uses Axum for routing.
//!
//! ## Module Organization
//!
//! - `ApiServer` — API server implementation
//! - `ApiConfig` — Server configuration
//! - Handlers — Request/response types
//!
//! ## Dependencies
//!
//! - `axum` — Web framework
//! - `tokio` — Async runtime
//! - `serde` / `serde_json` — Serialization
//! - `uuid` — UUID generation
//! - `anyhow` — Error handling
//! - `tracing` — Logging
//!
//! # Health check endpoint
//!
//! GET /health — Returns server status and version.
//!
//! # Status endpoints
//!
//! GET /api/status — Query race status.
//!
//! GET /api/best — Get best BPB results.
//!
//! # Control endpoints
//!
//! POST /api/race/start — Start a new race.
//!
//! ## Data Types
//!
//! Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Status response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Number of active workers.
    pub active_workers: u32,
    /// Total trials run so far.
    pub total_trials: u64,
    /// Total trials completed.
    pub completed: u64,
    /// Total trials pruned.
    pub pruned: u64,
    /// Current best BPB across all trials.
    pub best_bpb: f64,
    /// Best trial ID.
    pub best_trial_id: u64,
}

/// Start race request.
#[derive(Debug, Clone, Deserialize)]
pub struct StartRaceRequest {
    /// Number of OS threads to spawn.
    pub workers: u32,
    /// Trials each worker runs before joining.
    pub trials_per_worker: u32,
    /// Path of CSV telemetry sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_path: Option<String>,
    /// Base seed; worker `w` uses `base_seed.wrapping_add(w as u64)`.
    pub base_seed: u64,
    /// `d_model` value used by every trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_model: usize,
    /// Whether trials run in GF16 domain.
    pub use_gf16: bool,
}

/// Start race response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartRaceResponse {
    /// Race ID.
    pub race_id: String,
    /// Status.
    pub status: String,
}

/// Best trial result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestTrial {
    /// Trial ID
    pub trial_id: String,
    /// BPB
    pub bpb: f64,
    /// Seed
    pub seed: u64,
}

/// API server.
pub struct ApiServer {
    bind_addr: String,
    port: u16,
    /// Shared application state.
    state: tokio::sync::RwLock<ApiState>,
}

/// Application state.
#[derive(Debug, Clone, Default)]
pub struct ApiState {
    /// Currently active workers.
    pub active_workers: u32,
    /// Total trials run.
    pub total_trials: u64,
    /// Completed trials.
    pub completed: u64,
    /// Pruned trials.
    pub pruned: u64,
    /// Best BPB.
    pub best_bpb: f64,
    /// Best trial ID.
    pub best_trial_id: u64,
    /// Race ID for current race.
    pub race_id: Option<String>,
}

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Bind address.
    pub bind_addr: String,
    /// Port.
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

impl ApiServer {
    /// Create a new API server.
    pub fn new(config: ApiConfig) -> Self {
        Self {
            config,
            state: tokio::sync::RwLock::new(ApiState::default()),
        }
    }

    /// Run the API server.
    ///
    /// Binds TCP listener and starts Axum router.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.bind_addr, self.port);
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/api/status", get(status_handler))
            .route("/api/best", get(best_handler))
            .route("/api/race/start", post(start_race_handler))
            .route("/api/status", get(detailed_status_handler));

        info!("API server listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

// Health check handler.
async fn health_handler() -> axum::Json<HealthResponse> {
    axum::Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// Status handler.
async fn status_handler(State(state): State<Arc<tokio::sync::RwLock<ApiState>>>) -> axum::Json<StatusResponse> {
    let s = state.read().await;

    axum::Json(StatusResponse {
        active_workers: s.active_workers,
        total_trials: s.total_trials,
        completed: s.completed,
        pruned: s.pruned,
        best_bpb: s.best_bpb,
        best_trial_id: s.best_trial_id,
    })
}

/// Detailed status handler (for full state queries).
async fn detailed_status_handler(State(state): State<Arc<tokio::sync::RwLock<ApiState>>>) -> axum::Json<ApiState> {
    let s = state.read().await;

    axum::Json(s)
}

/// Best BPB handler.
async fn best_handler(State(state): State<Arc<tokio::sync::RwLock<ApiState>>>) -> axum::Json<BestTrial> {
    let s = state.read().await;

    // Mock: return top 3 results
    axum::Json(vec![
        BestTrial {
            trial_id: "mock-001".to_string(),
            bpb: 1.5,
            seed: 42,
        },
        BestTrial {
            trial_id: "mock-002".to_string(),
            bpb: 1.45,
            seed: 43,
        },
        BestTrial {
            trial_id: "mock-003".to_string(),
            bpb: 1.40,
            seed: 44,
        },
    ])
}

/// Start race handler.
async fn start_race_handler(
    State(state): State<Arc<tokio::sync::RwLock<ApiState>>>,
    Json(req): StartRaceRequest,
) -> axum::Json<StartRaceResponse> {
    let mut s = state.write().await;

    // Generate race ID
    let race_id = uuid::Uuid::new_v4().to_string();

    // Update state
    s.race_id = Some(race_id.clone());
    s.active_workers = req.workers;

    // Spawn placeholder workers (mock implementation)
    info!("Race started: {} workers, {} trials per worker", req.workers, req.trials_per_worker);

    // TODO: Actually spawn workers and execute trials

    // Mock: return success
    axum::Json(StartRaceResponse {
        race_id,
        status: "started".to_string(),
    })
}
