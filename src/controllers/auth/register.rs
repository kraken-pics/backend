use crate::{
    entity::{config as ConfigTable, user as UserTable},
    state::AppState,
    typings::{
        auth::IRegister,
        response::{ApiResponse, ErrorResponse},
    },
    util::{jwt::create_jwt, user},
};
use actix_identity::Identity;
use actix_web::{post, web, HttpMessage, HttpRequest, Result};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

// TO-DO:
// find out to create config at the same time as user
// with seaorm, as the current method is quite ugly
// also, find some way to improve creation speed
#[post("/register")]
async fn register(
    data: web::Json<IRegister>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<actix_web::web::Json<ApiResponse>, ErrorResponse> {
    // verify username restraints
    if let Err(err) = user::check_username(data.username.clone()) {
        return Err(ErrorResponse {
            message: err.to_string(),
        });
    }

    //verify password restraints
    if let Err(err) = user::check_password(data.password.clone()) {
        return Err(ErrorResponse {
            message: err.to_string(),
        });
    }

    // check if user exists in db, then error handle
    if UserTable::Entity::find()
        .filter(
            Condition::any()
                .add(UserTable::Column::Username.eq(data.username.clone()))
                .add(UserTable::Column::Email.eq(data.email.clone())),
        )
        .one(&state.db)
        .await
        .expect("user not found")
        .is_some()
    {
        return Err(ErrorResponse {
            message: "Username/email already used".to_string(),
        });
    };

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(data.password.clone().to_string().as_bytes(), &salt)
        .expect("Argon2id failure");

    let new_user = UserTable::ActiveModel {
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        password: Set(password_hash.to_string()),
        token: Set(Uuid::new_v4().clone().to_string()),
        uploadtoken: Set(Uuid::new_v4().clone().to_string()),
        ..Default::default()
    }
    .insert(&state.db)
    .await;
    if let Err(_) = new_user {
        return Err(ErrorResponse {
            message: "Internal error occurred, please try again".to_string(),
        });
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
        return Err(ErrorResponse {
            message: "Internal error occurred, please try again".to_string(),
        });
    };

    // set user in session
    if let Err(err) = Identity::login(
        &req.extensions(),
        create_jwt(new_user.clone().unwrap().token.to_string()).unwrap(),
    ) {
        return Err(ErrorResponse {
            message: err.to_string(),
        });
    };

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully registered user".to_string(),
    }))
}
