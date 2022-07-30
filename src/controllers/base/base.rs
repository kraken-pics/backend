use crate::typings::response::ApiResponse;

use actix_web::{get, Error, Responder, Result};

#[get("")]
async fn base() -> Result<impl Responder, Error> {
    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "API".to_string(),
    }))
}
