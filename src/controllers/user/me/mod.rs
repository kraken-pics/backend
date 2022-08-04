use actix_web::{web, Scope};

pub mod delete_user;
pub mod get_user;

// export @me's routes
pub fn routes() -> Scope {
    web::scope("/@me")
        .service(get_user::get_user)
        .service(delete_user::delete_user)
}
