use crate::{entity::user, state::state::AppState, util::response::ApiResponse};
use actix_web::{post, web, delete, Error, Responder, Result, Scope};
use bcrypt::hash;
use sea_orm::{prelude::Uuid, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

// global AppState
type AppData = web::Data<AppState>;


// export auth's routes
pub fn get() -> Scope {
    web::scope("/user").service(login).service(get_user).service(update_user).service(delete_user);
}

// get user route
#[get("")]
async fn get_user(data: web::Json<ILogin>, state: AppData) -> Result<impl Responder, Error> {

    
}

// update user route
#[post("")]
async fn update_user(data: web::Json<ILogin>, state: AppData) -> Result<impl Responder, Error> {

}

// delete user route
#[delete("")]
async fn delete_user(data: web::Json<IRegister>, state: AppData) -> Result<impl Responder, Error> {

}
