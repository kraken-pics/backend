use actix_web::{web, Scope};

pub mod login;
pub mod register;

pub fn routes() -> Scope {
    web::scope("/auth")
        .service(login::login)
        .service(register::register)
}
