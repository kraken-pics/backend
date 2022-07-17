use super::{auth, base, global, user};
use actix_web::{web, Scope};

// export base's routes
pub fn get() -> Scope {
    web::scope("")
        .service(base::routes::get())
        .service(user::routes::get())
        .service(global::routes::get())
        .service(auth::routes::get())
}
