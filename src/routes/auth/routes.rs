use crate::{entity::user, state::state::AppState, util::response::ApiResponse};
use actix_web::{post, web, Error, Responder, Result, Scope};
use bcrypt::hash;
use sea_orm::{prelude::Uuid, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
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
    // pub recaptcha: String,
}

// export auth's routes
pub fn get() -> Scope {
    web::scope("/auth").service(login).service(register)
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

#[post("")]
async fn register(data: web::Json<IRegister>, state: AppData) -> Result<impl Responder, Error> {
    // these if's are spaghetti
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
            message: "Password too short".to_string(),
        }));
    }

    if data.password.len() > 50 {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Password too long".to_string(),
        }));
    }

    if user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(data.username.clone()))
                .add(user::Column::Email.eq(data.email.clone())),
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

    let hashed_password = hash(data.password.clone(), 10).expect("Fail to hash");
    if let Err(_) = user::Entity::insert(user::ActiveModel {
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        password: Set(hashed_password.to_string()),
        token: Set(Uuid::new_v4().clone().to_string()),
        uploadtoken: Set(Uuid::new_v4().clone().to_string()),
        ..Default::default()
    })
    .exec(&state.db)
    .await
    {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Internal error occurred".to_string(),
        }));
    };

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "successfully registered user.".to_string(),
    }))
}
