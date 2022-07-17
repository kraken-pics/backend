use crate::typings::response::ApiResponse;

use actix_web::{get, web, Error, Responder, Result, Scope};

pub fn get() -> Scope {
    web::scope("/").service(get_base)
}

#[get("")]
async fn get_base() -> Result<impl Responder, Error> {
    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "API".to_string(),
    }))
}
