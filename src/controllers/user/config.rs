use crate::{
    entity::user as UserEntity,
    state::AppState,
    typings::response::{ConfigArgs, ConfigResponse, ErrorResponse},
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{get, web, Responder, Result};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[get("config")]
async fn download(
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
        }
    };

    Ok(web::Json(ConfigResponse {
        Arguments: ConfigArgs {
            upload_key: found_user.uploadtoken,
        },
        ..Default::default()
    }))
}
