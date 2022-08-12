use crate::{
    entity::user as UserTable,
    state::AppState,
    typings::{
        auth::ILogin,
        response::{ApiResponse, ErrorResponse},
    },
    util::jwt::create_jwt,
};
use actix_identity::Identity;
use actix_web::{post, web, HttpMessage, HttpRequest, Responder, Result};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[post("/login")]
async fn login(
    data: web::Json<ILogin>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, ErrorResponse> {
    // find user in db & check for non-existance
    let found_user = match UserTable::Entity::find()
        .filter(UserTable::Column::Username.eq(data.username.to_owned()))
        .one(&state.db)
        .await
        .expect("user not found")
    {
        Some(val) => val,
        None => {
            return Err(ErrorResponse {
                message: "Invalid username/password".to_string(),
            });
        }
    };

    let parsed_hash = PasswordHash::new(&found_user.password).unwrap();

    // check if stored hash compares successfully to the user provided password
    match Argon2::default()
        .verify_password(&data.password.clone().to_string().as_bytes(), &parsed_hash)
    {
        Ok(val) => val,
        Err(_) => {
            return Err(ErrorResponse {
                message: "Invalid username/password".to_string(),
            })
        }
    };

    // set user in session
    if let Err(err) = Identity::login(
        &req.extensions(),
        create_jwt(found_user.token.to_string()).unwrap(),
    ) {
        return Err(ErrorResponse {
            message: err.to_string(),
        });
    };

    // success
    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully logged in".to_string(),
    }))
}
