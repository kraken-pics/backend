use crate::{
    entity::user as UserTable,
    state::AppState,
    typings::{auth::ILogin, response::ApiResponse},
    util::jwt::create_jwt,
};
use actix_identity::Identity;
use actix_web::{post, web, Responder, Result};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[post("/login")]
async fn login(
    data: web::Json<ILogin>,
    state: web::Data<AppState>,
    id: Identity,
) -> Result<impl Responder> {
    // find user in db & check for non-existance
    let found_user = match UserTable::Entity::find()
        .filter(UserTable::Column::Username.eq(data.username.to_owned()))
        .one(&state.db)
        .await
        .expect("user not found")
    {
        Some(val) => val,
        None => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Invalid Username/password".to_string(),
            }))
        }
    };

    // check if stored hash compares successfully to the user provided password
    match bcrypt::verify(data.password.clone(), &found_user.password) {
        Ok(val) => val,
        Err(_) => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Invalid Username/password".to_string(),
            }))
        }
    };

    // set user in session
    id.remember(create_jwt(found_user.token.to_string()).unwrap());

    // success
    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully logged in".to_string(),
    }))
}
