// use crate::{entity::user, state::AppState, typings::response::ApiResponse, util::jwt::decode_jwt};
// use actix_identity::Identity;
// use actix_web::{get, post, web, Error, Responder, Result, Scope};
// use bcrypt::hash;
// use sea_orm::{prelude::Uuid, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
// use serde::Deserialize;

// // global AppState
// type AppData = web::Data<AppState>;

// #[derive(Deserialize)]
// pub struct IOauth {
//     pub name: String,
// }

// // export oauth's routes
// pub fn get() -> Scope {
//     web::scope("/oauth").service(authorize).service(create)
// }

// // authorize route
// #[get("/{oauth_id}/authorize")]
// async fn authorize(
//     data: actix_web::web::Path<i32>,
//     id: Identity,
//     state: AppData,
// ) -> Result<impl Responder, Error> {
//     let user_identity = match id.identity() {
//         Some(val) => decode_jwt(val),
//         None => {
//             return Ok(actix_web::web::Json(ApiResponse {
//                 success: false,
//                 message: "Not authorized".to_string(),
//             }));
//         }
//     };

//     if user_identity.is_err() {
//         return Ok(actix_web::web::Json(ApiResponse {
//             success: false,
//             message: "Invalid JWT Token".to_string(),
//         }));
//     }

//     let found_user = match user::Entity::find()
//         .filter(user::Column::Token.eq(user_identity.unwrap()))
//         .one(&state.db)
//         .await
//         .expect("User not found")
//     {
//         Some(val) => val,
//         None => {
//             return Ok(actix_web::web::Json(ApiResponse {
//                 success: false,
//                 message: "Not authorized".to_string(),
//             }));
//         }
//     };
//     return Ok(actix_web::web::Json(ApiResponse {
//         success: false,
//         message: data.to_string(),
//     }));
// }

// // create route
// #[post("/create")]
// async fn create(data: web::Json<IOauth>, state: AppData) -> Result<impl Responder, Error> {
//     return Ok(actix_web::web::Json(ApiResponse {
//         success: false,
//         message: "create".to_string(),
//     }));
// }
