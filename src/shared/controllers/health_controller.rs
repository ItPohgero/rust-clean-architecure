use rocket::serde::json::{Json, Value};
use serde_json::json;

#[get("/health")]
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}