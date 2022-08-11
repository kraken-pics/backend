use actix_web::{web, Scope};

pub mod delete;
pub mod get;

// export @me's routes
pub fn routes() -> Scope {
    web::scope("/@me").service(get::user).service(delete::user)
}
