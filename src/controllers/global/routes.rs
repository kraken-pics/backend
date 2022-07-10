use crate::{
    entity::{upload, user},
    state::AppState,
    typings::response::{Stats, StatsResponse},
};
use actix_web::{get, web, Error, Responder, Result, Scope};

use sea_orm::*;

type AppData = web::Data<AppState>;

pub fn get() -> Scope {
    web::scope("/global").service(get_statistics)
}

#[get("/statistics")]
async fn get_statistics(state: AppData) -> Result<impl Responder, Error> {
    let count_user = user::Entity::find()
        .count(&state.db)
        .await
        .expect("Failed to count users");
    let count_upload = upload::Entity::find()
        .count(&state.db)
        .await
        .expect("Failed to count uploads");

    return Ok(actix_web::web::Json(StatsResponse {
        success: true,
        message: "Successfully grabbed statistics".to_string(),
        statistics: Stats {
            users: count_user,
            uploads: count_upload,
        },
    }));
}
