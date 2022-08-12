use crate::entity::sea_orm_active_enums::{Membership, Role};
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

use crate::entity::upload as UploadEntity;

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Display)]
pub struct ErrorResponse {
    pub message: String,
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ApiResponse {
                success: false,
                message: self.to_string(),
            })
    }
}

#[derive(Serialize)]
pub struct Stats {
    pub users: usize,
    pub uploads: usize,
    pub storage: u64,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub message: String,
    pub statistics: Stats,
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub role: Role,
    pub membership: Membership,
    pub uploadtoken: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<User>,
}

#[derive(Serialize)]
pub struct GalleryResponse {
    pub success: bool,
    pub message: String,
    pub uploads: Option<Vec<UploadEntity::Model>>,
}

#[derive(Serialize)]
pub struct ConfigArgs {
    pub upload_key: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ConfigResponse {
    pub Version: String,
    pub Name: String,
    pub DestinationType: String,
    pub RequestMethod: String,
    pub RequestURL: String,
    pub Body: String,
    pub Arguments: ConfigArgs,
    pub FileFormName: String,
    pub URL: String,
}

impl Default for ConfigResponse {
    fn default() -> ConfigResponse {
        let api_url = dotenv::var("API_URL").expect("API_URL envar");

        ConfigResponse {
            Version: "14.1.0".to_owned(),
            Name: "Kraken.pics".to_owned(),
            DestinationType: "ImageUploader".to_owned(),
            RequestMethod: "POST".to_owned(),
            RequestURL: format!("{}/upload", api_url),
            Body: "MultipartFormData".to_owned(),
            Arguments: ConfigArgs {
                upload_key: "key".to_string(),
            },
            FileFormName: "file".to_owned(),
            URL: "{json:message}".to_owned(),
        }
    }
}
