use actix_web::{web::Json, HttpResponse, Result};

use serde::Serialize;

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>>
where
    T: Serialize,
{
    Ok(Json(data))
}

// / Helper function to reduce boilerplate of an empty OK response
pub fn respond_ok() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(""))
}

// pub fn respond_str(inp: String) -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().body(inp))
// }
