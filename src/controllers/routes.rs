use super::{auth, base, global, upload, user};
use actix_web::{web, Scope};

// export all routes
pub fn get() -> Scope {
    web::scope("")
        .service(base::routes())
        .service(global::routes())
        .service(auth::routes())
        .service(user::routes())
        .service(upload::routes())
}
