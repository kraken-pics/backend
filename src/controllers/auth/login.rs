use crate::{
    entity::user as UserTable,
    state::AppState,
    typings::{auth::ILogin, response::ApiResponse},
    util::jwt::create_jwt,
};
use actix_identity::Identity;
use actix_web::{
    error::ResponseError, http::header::ContentType, post, web, HttpResponse, Responder, Result,
};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use derive_more::Display;

#[derive(Debug, Display)]
pub struct TaskError {
    message: String,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ApiResponse {
                success: false,
                message: self.to_string(),
            })
    }
}

#[post("/login")]
async fn login(
    data: web::Json<ILogin>,
    state: web::Data<AppState>,
    id: Identity,
) -> Result<impl Responder, TaskError> {
    // find user in db & check for non-existance
    let found_user = match UserTable::Entity::find()
        .filter(UserTable::Column::Username.eq(data.username.to_owned()))
        .one(&state.db)
        .await
        .expect("user not found")
    {
        Some(val) => val,
        None => {
            return Err(TaskError {
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
            return Err(TaskError {
                message: "Invalid username/password".to_string(),
            })
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
