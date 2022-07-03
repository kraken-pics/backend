use crate::{entity::user, state::state::AppState, util::response::ApiResponse};
use actix_web::{post, web, Error, Responder, Result, Scope};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::Deserialize;

// global AppState
type AppData = web::Data<AppState>;

#[derive(Deserialize)]
pub struct ILogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct IRegister {
    pub username: String,
    pub email: String,
    pub password: String,
    pub recaptcha: String,
}

// export auth's routes
pub fn get() -> Scope {
    web::scope("/auth").service(login)
}

// login route
#[post("/")]
async fn login(data: web::Json<ILogin>, state: AppData) -> Result<impl Responder, Error> {
    let find_user = user::Entity::find()
        .filter(user::Column::Username.eq(data.username.to_owned()))
        .one(&state.db)
        .await
        .expect("User not found");
    let found_user = match find_user {
        Some(val) => val,
        None => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Invalid username/password".to_string(),
            }))
        }
    };

    let verify_pass = bcrypt::verify(data.password.clone(), &found_user.password);
    if !verify_pass.is_ok() {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Invalid username/password".to_string(),
        }));
    }

    return Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: found_user.username.to_string(),
    }));
}

#[post("/register")]
async fn register(data: web::Json<IRegister>, state: AppData) -> Result<impl Responder, Error> {
    match recaptcha::verify(
        dotenv::var("RECAPTCHA_SECRET")
            .expect("RECAPTCHA_SECRET not set in env")
            .as_str(),
        &data.recaptcha,
        None,
    )
    .await
    {
        Ok(_) => (),
        Err(err) => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: err.to_string(),
            }))
        }
    };

    if data.username.len() < 3 {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Username too short".to_string(),
        }));
    }

    if data.username.len() > 30 {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Username too long".to_string(),
        }));
    }

    if data.password.len() < 5 {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Username too short".to_string(),
        }));
    }

    if user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(data.username.to_owned()))
                .add(user::Column::Email.eq(data.email.to_owned())),
        )
        .one(&state.db)
        .await
        .expect("User not found")
        .is_some()
    {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Username/email already used".to_string(),
        }));
    };

    user::Entity::insert(user::ActiveModel {
        username: todo!(""),
        email: todo!(""),
        password: todo!(""),
        token: todo!(""),
        uploadtoken: todo!(""),
        ..Default::default()
    });

    Ok(actix_web::web::Json(ApiResponse {
        success: false,
        message: "meow".to_string(),
    }))
}
