use actix_web::{web, Scope};
pub mod base;

pub fn routes() -> Scope {
    web::scope("/").service(base::base)
}
