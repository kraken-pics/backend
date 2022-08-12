use crate::{
    entity::user as UserEntity,
    state::AppState,
    typings::response::{ApiResponse, ErrorResponse},
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{delete, web, Responder, Result};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

// delete user route
#[delete("")]
async fn user(
    state: web::Data<AppState>,
    user: Option<Identity>,
) -> Result<impl Responder, ErrorResponse> {
    let user_identity = match user {
        Some(val) => decode_jwt(val.id().unwrap()),
        None => {
            return Err(ErrorResponse {
                message: "Not authorized".to_string(),
            })
        }
    };

    if user_identity.is_err() {
        return Err(ErrorResponse {
            message: "Invalid JWT Token".to_string(),
        });
    }

    let found_user = match UserEntity::Entity::find()
        .filter(UserEntity::Column::Token.eq(user_identity.unwrap()))
        .one(&state.db)
        .await
        .expect("User not found")
    {
        Some(val) => val,
        None => {
            return Err(ErrorResponse {
                message: "Not authorized".to_string(),
            });
        }
    };

    match UserEntity::Entity::delete_by_id(found_user.id)
        .exec(&state.db)
        .await
    {
        Ok(val) => val,
        Err(_) => {
            return Err(ErrorResponse {
                message: "An unknown error occurred, please try again".to_string(),
            });
        }
    };

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully deleted user!".to_string(),
    }))
}
