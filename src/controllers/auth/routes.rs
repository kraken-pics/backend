use crate::{
    entity::user,
    state::AppState,
    typings::{
        auth::{ILogin, IRegister},
        response::ApiResponse,
    },
    util::jwt::create_jwt,
};
use actix_identity::Identity;
use actix_web::{post, web, Error, Responder, Result, Scope};
use bcrypt::hash;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set,
};

type AppData = web::Data<AppState>;

pub fn get() -> Scope {
    web::scope("/auth").service(login).service(register)
}

#[post("/login")]
async fn login(
    data: web::Json<ILogin>,
    state: AppData,
    id: Identity,
) -> Result<impl Responder, Error> {
    let found_user = match user::Entity::find()
        .filter(user::Column::Username.eq(data.username.to_owned()))
        .one(&state.db)
        .await
        .expect("User not found")
    {
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

    // set user in session
    id.remember(create_jwt(found_user.token.to_string()).unwrap());

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully logged in".to_string(),
    }))
}

#[post("/register")]
async fn register(
    data: web::Json<IRegister>,
    state: AppData,
    id: Identity,
) -> Result<impl Responder, Error> {
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

    let hashed_password = match hash(data.password.clone(), 10) {
        Ok(val) => val,
        Err(_) => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Internal error occurred, please try again".to_string(),
            }));
        }
    };

    let new_user = user::ActiveModel {
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        password: Set(hashed_password.to_string()),
        token: Set(Uuid::new_v4().clone().to_string()),
        uploadtoken: Set(Uuid::new_v4().clone().to_string()),
        ..Default::default()
    }
    .insert(&state.db)
    .await;

    match new_user {
        Ok(_) => (),
        Err(_) => {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Internal error occurred, please try again".to_string(),
            }));
        }
    }

    id.remember(create_jwt(new_user.unwrap().username.to_string()).unwrap());

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully registered user".to_string(),
    }))
}
