use crate::{
    entity::user as UserEntity, state::AppState, typings::response::ApiResponse,
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{delete, web, Error, Responder, Result};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

// delete user route
#[delete("")]
async fn user(state: web::Data<AppState>, id: Identity) -> Result<impl Responder, Error> {
    let user_identity = match id.identity() {
        Some(val) => decode_jwt(val),
        None => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Not authorized".to_string(),
            }));
        }
    };

    if user_identity.is_err() {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Invalid JWT Token".to_string(),
        }));
    }

    let found_user = match UserEntity::Entity::find()
        .filter(UserEntity::Column::Token.eq(user_identity.unwrap()))
        .one(&state.db)
        .await
        .expect("User not found")
    {
        Some(val) => val,
        None => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Not authorized".to_string(),
            }));
        }
    };

    match UserEntity::Entity::delete_by_id(found_user.id)
        .exec(&state.db)
        .await
    {
        Ok(val) => val,
        Err(_) => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: true,
                message: "An unknown error occurred, please try again".to_string(),
            }));
        }
    };

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully deleted user!".to_string(),
    }))
}
