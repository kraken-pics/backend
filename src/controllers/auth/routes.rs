use crate::{
    entity::{config as ConfigTable, user as UserTable},
    state::AppState,
    typings::{
        auth::{ILogin, IRegister},
        response::ApiResponse,
    },
    util::{jwt::create_jwt, user},
};
use actix_identity::Identity;
use actix_web::{post, web, Error, Responder, Result, Scope};
use bcrypt::hash;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set,
};

pub fn get() -> Scope {
    web::scope("/auth").service(login).service(register)
}

#[post("/login")]
async fn login(
    data: web::Json<ILogin>,
    state: web::Data<AppState>,
    id: Identity,
) -> Result<impl Responder, Error> {
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

// TO-DO:
// find out to create config at the same time as user
// with seaorm, as the current method is quite ugly
// also, find some way to improve creation speed
#[post("/register")]
async fn register(
    data: web::Json<IRegister>,
    state: web::Data<AppState>,
    id: Identity,
) -> Result<impl Responder, Error> {
    // these if's are spaghetti

    if let Err(err) = user::check_username(data.username.clone()) {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: err.to_string(),
        }));
    }

    if UserTable::Entity::find()
        .filter(
            Condition::any()
                .add(UserTable::Column::Username.eq(data.username.to_owned()))
                .add(UserTable::Column::Email.eq(data.email.to_owned())),
        )
        .one(&state.db)
        .await
        .expect("user not found")
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

    let new_user = UserTable::ActiveModel {
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        password: Set(hashed_password.to_string()),
        token: Set(Uuid::new_v4().clone().to_string()),
        uploadtoken: Set(Uuid::new_v4().clone().to_string()),
        ..Default::default()
    }
    .insert(&state.db)
    .await;
    if let Err(_) = new_user {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Internal error occurred, please try again".to_string(),
        }));
    }

    // create config, no need for un-joined if as we don't require
    // any values from this table after creation
    if let Err(_) = (ConfigTable::ActiveModel {
        userid: Set(new_user.clone().unwrap().id),
        ..Default::default()
    }
    .insert(&state.db)
    .await)
    {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Internal error occurred, please try again".to_string(),
        }));
    };

    id.remember(create_jwt(new_user.clone().unwrap().token.to_string()).unwrap());

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully registered user".to_string(),
    }))
}
