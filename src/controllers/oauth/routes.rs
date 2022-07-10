use crate::{entity::user, state::state::AppState, util::response::ApiResponse};
use actix_web::{post, web, Error, Responder, Result, Scope};
use bcrypt::hash;
use sea_orm::{prelude::Uuid, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

// global AppState
type AppData = web::Data<AppState>;

#[derive(Deserialize)]
pub struct IOauth {
    pub name: String,
}

// export oauth's routes
pub fn get() -> Scope {
    web::scope("/oauth").service(authorize).service(create)
}

// authorize route
#[get("/{oauth_id}/authorize")]
async fn authorize(state: AppData) -> Result<impl Responder, Error> {
    return Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "authorize".to_string(),
    }));
}

// create route
#[post("/create")]
async fn create(data: web::Json<IOauth>, state: AppData) -> Result<impl Responder, Error> {
    return Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "create".to_string(),
    }));
}

