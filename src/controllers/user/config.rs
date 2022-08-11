use crate::{
    entity::user as UserEntity,
    state::AppState,
    typings::response::{ApiResponse, ConfigArgs, ConfigResponse, User, UserResponse},
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{error, get, web, Responder, Result};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "error occurred: {}", message)]
struct MyError {
    success: bool,
    message: String,
}

impl error::ResponseError for MyError {}

#[get("config")]
async fn download(state: web::Data<AppState>, id: Identity) -> Result<impl Responder, MyError> {
    let user_identity = match id.identity() {
        Some(val) => decode_jwt(val),
        None => {
            return Err(MyError {
                success: false,
                message: "Not authorized".to_string(),
            });
        }
    };

    if user_identity.is_err() {
        return Err(MyError {
            success: false,
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
            return Err(MyError {
                success: false,
                message: "Not authorized".to_string(),
            });
        }
    };

    Ok(web::Json(ConfigResponse {
        Arguments: ConfigArgs {
            upload_key: found_user.uploadtoken,
        },
        ..Default::default()
    }))
}
