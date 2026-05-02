[workspace]
members = [
    "trios-algorithm-arena",
    "trios-igla-race-pipeline",
    "trios-igla-race-hack",
]
exclude = [
    "trios-algorithm-arena",
    "trios-igla-race-pipeline",
    "trios-igla-race-hack",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Dmitrii Vasilev"]
license = "MIT"
repository = "https://github.com/gHashTag/trios"

[workspace.dependencies]
tower-http = { version = "0.5", features = ["cors"] }
base64 = "0.22"
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
axum = { version = "0.7", features = ["ws"] }
chrono = { version = "0.4", features = ["serde"] }
clap = { version = "4", features = ["derive"] }
tokio-tungstenite = { version = "0.24", features = ["futures", "http"] }

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
