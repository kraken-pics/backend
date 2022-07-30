use actix_multipart_extract::{MultipartConfig, MultipartError};
use actix_web::HttpResponse;
use sea_orm::DatabaseConnection;

use crate::{db::db, typings::response::ApiResponse};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

fn error_handler(err: MultipartError) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiResponse {
        success: false,
        message: err.to_string(),
    })
}

impl AppState {
    pub async fn init_db() -> Self {
        let result = Self {
            db: db::get_db().await,
        };

        return result;
    }
    pub async fn init_multipart() -> MultipartConfig {
        MultipartConfig::set_error_handler(Default::default(), error_handler)
    }
}
