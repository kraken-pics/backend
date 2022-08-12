use crate::{
    entity::{upload as UploadTable, user as UserTable},
    state::AppState,
    typings::response::{ErrorResponse, Stats, StatsResponse},
};
use actix_web::{get, web, Error, Responder};

use sea_orm::*;

use fs_extra::dir::get_size;

async fn get_stats(state: web::Data<AppState>) -> Result<(usize, usize, u64), ()> {
    let upload_dir = dotenv::var("UPLOAD_DIR").unwrap();

    let count_user = UserTable::Entity::find().count(&state.db).await;
    let count_upload = UploadTable::Entity::find().count(&state.db).await;
    let storage_used = get_size(upload_dir);

    Ok((
        count_user.unwrap(),
        count_upload.unwrap(),
        storage_used.unwrap(),
    ))
}

#[get("/statistics")]
async fn statistics(state: web::Data<AppState>) -> Result<impl Responder, ErrorResponse> {
    let (users, uploads, storage) = get_stats(state).await.unwrap();

    Ok(actix_web::web::Json(StatsResponse {
        success: true,
        message: "Successfully grabbed statistics".to_string(),
        statistics: Stats {
            users: users,
            uploads: uploads,
            storage: storage,
        },
    }))
}
