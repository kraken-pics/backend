use super::{auth, base, global, upload, user};
use actix_web::{web, Scope};

// export all routes
pub fn get() -> Scope {
    web::scope("")
        .service(base::routes::get())
        .service(global::routes::get())
        .service(auth::routes::get())
        .service(user::routes::get())
        .service(upload::routes::get())
}
