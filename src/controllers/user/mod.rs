use actix_web::{web, Scope};

pub mod gallery;
pub mod me;

// export user's routes
pub fn routes() -> Scope {
    web::scope("/user").service(gallery::gallery)
}
