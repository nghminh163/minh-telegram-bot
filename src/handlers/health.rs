use crate::helpers::respond_json;
use actix_web::{web::Json, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Handler to get the liveness of the service
pub async fn get_health() -> Result<Json<HealthResponse>> {
    respond_json::<HealthResponse>(HealthResponse {
        status: "ok".into(),
    })
}
