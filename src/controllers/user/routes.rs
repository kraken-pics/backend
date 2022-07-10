use crate::{
    entity::user,
    state::AppState,
    typings::response::{ApiResponse, User, UserResponse},
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{delete, get, post, web, Error, Responder, Result, Scope};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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
async fn get_user(state: AppData, id: Identity) -> Result<impl Responder, Error> {
    let user_identity = match id.identity() {
        Some(val) => decode_jwt(val),
        None => {
            return Ok(actix_web::web::Json(UserResponse {
                success: false,
                message: "Not authorized".to_string(),
                user: None,
            }));
        }
    };

    if user_identity.is_err() {
        return Ok(actix_web::web::Json(UserResponse {
            success: false,
            message: "Invalid JWT Token".to_string(),
            user: None,
        }));
    }

    let found_user = match user::Entity::find()
        .filter(user::Column::Token.eq(user_identity.unwrap()))
        .one(&state.db)
        .await
        .expect("User not found")
    {
        Some(val) => val,
        None => {
            return Ok(actix_web::web::Json(UserResponse {
                success: false,
                message: "Not authorized".to_string(),
                user: None,
            }));
        }
    };

    return Ok(actix_web::web::Json(UserResponse {
        success: true,
        message: "Successfully found current user!".to_string(),
        user: Some(User {
            username: found_user.username.to_owned(),
            email: found_user.email.to_owned(),
            role: found_user.role.to_owned(),
            membership: found_user.membership.to_owned(),
            uploadtoken: found_user.uploadtoken.to_owned(),
        }),
    }));
}

// update user route
#[post("")]
async fn update_user(_state: AppData) -> Result<impl Responder, Error> {
    Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "update".to_string(),
    }))
}

// delete user route
#[delete("")]
async fn delete_user(state: AppData, id: Identity) -> Result<impl Responder, Error> {
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

    let found_user = match user::Entity::find()
        .filter(user::Column::Token.eq(user_identity.unwrap()))
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

    match user::Entity::delete_by_id(found_user.id)
        .exec(&state.db)
        .await
    {
        Ok(val) => val,
        Err(_) => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: true,
                message: "Unknown error occurred, please try again".to_string(),
            }));
        }
    };

    return Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully deleted user!".to_string(),
    }));
}
