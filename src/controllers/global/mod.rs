use actix_web::{web, Scope};

pub mod statistics;

pub fn routes() -> Scope {
    web::scope("/global").service(statistics::statistics)
}
