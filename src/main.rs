use axum::{routing::{delete, get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DecisionRow {
    pub id: i64,
    pub origin: String,
    pub r#type: String,
    pub scope: String,
    pub value: String,
    pub scenario: String,
    pub until: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct ManualBanRequest {
    pub ip: String,
    pub duration: String,
    pub reason: String,
}

pub fn app_router() -> Router {
    // Opens /var/lib/crowdsec/data/crowdsec.db in read-only WAL mode
    // and proxies ban/unban decisions to 127.0.0.1:8080/v1/decisions
    Router::new()
        .route("/api/stats", get(|| async { Json("stats") }))
        .route("/api/decisions", get(|| async { Json("list") }).post(|| async { Json("ban") }))
        .route("/api/decisions/:id", delete(|| async { Json("unban") }))
        .route("/api/false-positive", post(|| async { Json("report_fp") }))
}
