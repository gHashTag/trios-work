//! Dashboard — Live dashboard with event streaming.
//!
//! Provides a WebSocket-based live dashboard that streams IGLA RACE
//! events in real-time. Uses SSE for server-sent events.
//!
//! ## Module Organization
//!
//! - `Dashboard` — Dashboard configuration and state
//! - `DashboardEvent` — Event types (trial_registered, checkpoint, etc.)
//! - `spawn_dashboard` — WebSocket server for streaming
//!
//! ## External Dependencies
//!
//! - `tokio` — Async runtime
//! - `axum` — Web framework
//! - `uuid` — UUID generation
//! - `serde` / `serde_json` — Serialization
//! - `anyhow` — Error handling
//! - `tracing` — Logging

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio_streamwrappers::WebSocketStream;
use tokio_tungstenite::Tungstenite;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::neon::{NeonDb, NeonDbClient, spawn_heartbeat};

/// Dashboard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Bind address for HTTP server.
    pub bind_addr: String,
    /// Port for HTTP server.
    pub port: u16,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

/// Dashboard event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DashboardEvent {
    /// Trial registered in database.
    TrialRegistered {
        trial_id: String,
        agent_id: String,
        branch: String,
    },
    /// Checkpoint recorded.
    Checkpoint {
        trial_id: String,
        rung: u32,
        bpb: f64,
        worker_id: u32,
    },
    /// Trial completed.
    TrialCompleted {
        trial_id: String,
        final_bpb: f64,
        steps: u32,
        worker_id: u32,
    },
    /// Trial pruned.
    TrialPruned {
        trial_id: String,
        rung: u32,
        bpb: f64,
        worker_id: u32,
    },
    /// Worker joined pool.
    WorkerJoined {
        worker_id: u32,
    },
    /// Worker left pool.
    WorkerLeft {
        worker_id: u32,
    },
    /// New leader candidate.
    NewLeader {
        trial_id: String,
        bpb: f64,
    },
    /// Leader updated.
    LeaderUpdated {
        trial_id: String,
        old_bpb: f64,
        new_bpb: f64,
    },
    /// Race started.
    RaceStarted {
        pool_id: String,
        workers: u32,
        trials_per_worker: u32,
    },
    /// Race completed.
    RaceCompleted {
        pool_id: String,
        total_trials: u64,
        completed: u64,
        pruned: u64,
        best_bpb: f64,
    },
}

/// Dashboard state shared across WebSocket connections.
#[derive(Debug)]
pub struct DashboardState {
    /// Connected clients.
    clients: Vec<String>,
    /// Current race pool being tracked.
    active_pool_id: Option<String>,
    /// Event buffer for streaming to clients.
    event_queue: VecDeque<DashboardEvent>,
    /// Last event timestamp.
    last_event_time: Option<Instant>,
}

/// Shared dashboard state.
static DASHBOARD_STATE: tokio::sync::RwLock<DashboardState> = tokio::sync::RwLock::new(DashboardState {
    clients: Vec::new(),
    active_pool_id: None,
    event_queue: VecDeque::new(),
    last_event_time: None,
});

/// Dashboard server with SSE streaming.
pub struct Dashboard {
    config: DashboardConfig,
    state: RwLock<DashboardState>,
    /// Stop signal for shutdown.
    shutdown_tx: tokio::sync::broadcast::Sender<()>,
}

impl Dashboard {
    /// Create a new dashboard server.
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            config,
            state: RwLock::new(DashboardState {
                clients: Vec::new(),
                active_pool_id: None,
                event_queue: VecDeque::new(),
                last_event_time: None,
            }),
            shutdown_tx: tokio::sync::broadcast::channel(1),
        }
    }

    /// Start the dashboard server.
    ///
    /// Binds WebSocket server and starts accepting connections.
    /// Returns `Ok(handle)` where handle can be used to join the shutdown channel.
    pub async fn start(&self) -> Result<impl Stream<Item = Result<tungstenite::Message<Result<tungstenite::Message<()>, tungstenite::Message<()>, tungstenite::Message<()>>>> {
        let addr = format!("{}:{}", self.config.bind_addr, self.config.port);
        info!("Starting IGLA RACE dashboard on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;

        // Spawn heartbeat task
        let db = NeonDbClient::new().await?;
        spawn_heartbeat(db);

        let state = self.state.clone();
        {
            let mut state = state.write().await;
            state.clients.push(format!("dashboard-{}", std::process::id()));

            let (event_stream, handle) = tungstenite::accept_async(listener).await?;

            // Spawn handler task
            let db_clone = db.clone();
            let state_clone = state.clone();
            let shutdown_rx = self.shutdown_tx.subscribe();

            tokio::spawn(async move {
                use tungstenite::Message::*;
                use tungstenite_tungstenite::Message as TungsteniteMessage;

                info!("Dashboard client connected: {:?}", handle.peer_addr());

                while !shutdown_rx.is_empty() {
                    // Handle incoming message
                    match event_stream.try_next().await {
                        Ok(Some(msg)) => {
                            // Server message
                            if let TungsteniteMessage::Text(text) = msg {
                                // Client sent command
                                if text.starts_with("/") {
                                    handle_command(text.trim(), &mut state, &db_clone, &state_clone).await;
                                }
                            }
                            } else {
                                // Client sent JSON (ignore for now)
                                let _ = text;
                            }
                        }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            error!("WebSocket error: {:?}", e);
                            break;
                        }
                    }
                }

                // Spawn event broadcaster
                let mut broadcast_interval = tokio::time::interval(Duration::from_secs(1));
                let shutdown_rx_clone = shutdown_rx.resubscribe();
                loop {
                    tokio::select! {
                        _ = broadcast_interval.tick() => {
                            // Broadcast queued events
                            let events: Vec::new();
                            {
                                let s = state.read().await;
                                events.push(DashboardEvent::TrialRegistered {
                                    trial_id: "sample-001".to_string(),
                                    agent_id: "w0".to_string(),
                                    branch: "main".to_string(),
                                });
                            }
                            }
                            if !events.is_empty() {
                                let s = state.read().await;
                                for client in s.clients.iter() {
                                    let _ = client.clone();
                                    if let Err(e) = event_stream.send(TungsteniteMessage::events(events.to_vec()).await) {
                                        error!("Send error to {}: {}", _);
                                    }
                                }
                            }
                        }

                        _ = shutdown_rx_clone.recv() => {
                            // Shutdown signal received
                            info!("Shutdown signal received");
                            break;
                        }
                    }
                }
            });

            info!("Dashboard client disconnected: {:?}", handle.peer_addr());

            Ok(())
        }

    /// Stop the dashboard server.
    ///
    /// Broadcasts shutdown signal and closes all connections.
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping IGLA RACE dashboard");
        self.shutdown_tx.send(()).await?;

        // Give time for shutdown to propagate
        tokio::time::sleep(Duration::from_secs(2)).await;

        Ok(())
    }
}

/// Handle CLI command (for future extension).
async fn handle_command(
    cmd: &str,
    state: &mut DashboardState,
    db: &NeonDb,
    _state: &DashboardState,
    _db: &NeonDb,
) {
    match cmd {
        "start" => {
            info!("Command: start (dashboard)");
        }
        "stop" => {
            info!("Command: stop (dashboard)");
        }
        "status" => {
            info!("Command: status (dashboard)");
        }
        "best" => {
            info!("Command: best (dashboard)");
        }
        _ => {
            info!("Unknown command: {}", cmd);
        }
    }
}

/// Broadcast event.
fn broadcast_event(state: &DashboardState, event: DashboardEvent) {
    let s = state.read().await;
    s.event_queue.push_back(event);
    s.last_event_time = Some(Instant::now());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_config_default() {
        let cfg = DashboardConfig::default();
        assert_eq!(cfg.bind_addr, "127.0.0.1");
        assert_eq!(cfg.port, 8080);
    }

    #[test]
    fn test_dashboard_state_initial() {
        let state = DashboardState::default();
        assert!(state.clients.is_empty());
        assert!(state.active_pool_id.is_none());
    }

    #[test]
    fn test_event_roundtrip() {
        let state = DashboardState::default();
        let mut clients = state.clients.write().await;
        clients.push("client-1".to_string());
        clients.push("client-2".to_string());

        let event_queue = state.event_queue.clone();
        broadcast_event(&state, DashboardEvent::TrialRegistered {
            trial_id: "test-001".to_string(),
            agent_id: "w1".to_string(),
            branch: "test".to_string(),
        });

        drop(clients);

        let s = state.read().await;
        assert_eq!(s.event_queue.len(), 1);
        assert_eq!(s.event_queue.front(), DashboardEvent::TrialRegistered {
            trial_id: "test-001".to_string(),
            agent_id: "w1".to_string(),
            branch: "test".to_string(),
        });
    }
}
