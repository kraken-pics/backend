use actix_web::{web, Scope};

pub mod upload;

pub fn routes() -> Scope {
    web::scope("/upload").service(upload::upload)
}
