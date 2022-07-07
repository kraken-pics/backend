use crate::{entity::user, state::state::AppState, util::response::ApiResponse};
use actix_web::{delete, get, post, web, Error, Responder, Result, Scope};
use sea_orm::{Condition, EntityTrait};

// global AppState
type AppData = web::Data<AppState>;

// export auth's routes
pub fn get() -> Scope {
    web::scope("/user")
        .service(get_user)
        .service(update_user)
        .service(delete_user)
}

// get user route
#[get("")]
async fn get_user(state: AppData) -> Result<impl Responder, Error> {
    let find_user = user::Entity::find()
        .one(&state.db)
        .await
        .expect("User not found");
    return Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "get".to_string(),
    }));
}

// update user route
#[post("")]
async fn update_user(state: AppData) -> Result<impl Responder, Error> {
    Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "update".to_string(),
    }))
}

// delete user route
#[delete("")]
async fn delete_user(state: AppData) -> Result<impl Responder, Error> {
    Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "delete".to_string(),
    }))
}
