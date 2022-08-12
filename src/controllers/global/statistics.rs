use crate::{
    entity::{upload as UploadTable, user as UserTable},
    state::AppState,
    typings::response::{Stats, StatsResponse},
};
use actix_web::{get, web, Responder};

use sea_orm::*;

use fs_extra::dir::get_size;

#[get("/statistics")]
async fn statistics(state: web::Data<AppState>) -> impl Responder {
    let upload_dir = dotenv::var("UPLOAD_DIR").expect("UPLOAD_DIR must be set");

    let count_user = UserTable::Entity::find()
        .count(&state.db)
        .await
        .expect("Failed to count users");
    let count_upload = UploadTable::Entity::find()
        .count(&state.db)
        .await
        .expect("Failed to count uploads");
    let storage_used = get_size(upload_dir).unwrap();

    actix_web::web::Json(StatsResponse {
        success: true,
        message: "Successfully grabbed statistics".to_string(),
        statistics: Stats {
            users: count_user,
            uploads: count_upload,
            storage: storage_used,
        },
    })
}
