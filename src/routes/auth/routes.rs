use crate::{entity::user, state::state::AppState, util::response::ApiResponse};
use actix_web::{post, web, Error, Responder, Result, Scope};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

// global AppState
type AppData = web::Data<AppState>;

#[derive(Deserialize)]
pub struct ILogin {
    pub username: String,
    pub password: String,
}

// export auth's routes
pub fn get() -> Scope {
    web::scope("/auth").service(login)
}

type Response<T> = Result<T, Error>;

// login route
#[post("/")]
async fn login(data: web::Json<ILogin>, state: AppData) -> Response<impl Responder> {
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
