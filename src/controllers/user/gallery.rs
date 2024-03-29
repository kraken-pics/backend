use crate::{
    entity::upload as UploadEntity,
    entity::user as UserEntity,
    state::AppState,
    typings::response::{ErrorResponse, GalleryResponse},
    util::jwt::decode_jwt,
};
use actix_identity::Identity;
use actix_web::{get, web, Responder, Result};

use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

#[get("/gallery/{page}")]
async fn gallery(
    state: web::Data<AppState>,
    user: Option<Identity>,
    page: web::Path<usize>,
) -> Result<impl Responder, ErrorResponse> {
    let images_per_page = dotenv::var("IMAGES_PER_PAGE").expect("IMAGES_PER_PAGE envar");

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
            .unwrap();
        }
    };

    let found_uploads = UploadEntity::Entity::find()
        .filter(UploadEntity::Column::Userid.eq(found_user.id.clone()))
        .paginate(&state.db, images_per_page.parse::<usize>().unwrap())
        .fetch_page(*page)
        .await;

    Ok(actix_web::web::Json(GalleryResponse {
        success: true,
        message: "Successfully found current user!".to_string(),
        uploads: Some(found_uploads.unwrap()),
    }))
}
