use crate::{
    entity::user as UserEntity,
    state::AppState,
    typings::response::{ErrorResponse, User, UserResponse},
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{error::ParseError, get, web, Responder, Result};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[get("")]
async fn user(state: web::Data<AppState>, id: Identity) -> Result<impl Responder, ParseError> {
    let user_identity = match id.identity() {
        Some(val) => decode_jwt(val),
        None => {
            return Err(ErrorResponse {
                message: "Not authorized".to_string(),
            })
            .unwrap();
        }
    };

    if user_identity.is_err() {
        return Err(ErrorResponse {
            message: "Invalid JWT Token".to_string(),
        })
        .unwrap();
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
            })
            .unwrap();
        }
    };

    Ok(actix_web::web::Json(UserResponse {
        success: true,
        message: "Successfully found current user!".to_string(),
        user: Some(User {
            username: found_user.username.to_owned(),
            email: found_user.email.to_owned(),
            role: found_user.role.to_owned(),
            membership: found_user.membership.to_owned(),
            uploadtoken: found_user.uploadtoken.to_owned(),
        }),
    }))
}
