use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::guard::GuardContext;
use actix_web::{guard, HttpServer};
use actix_web::{web, App};
use dotenv::dotenv;

use kraken::controllers;
use kraken::state;

fn protect_routes(req: &GuardContext) -> bool {
    let guard_secret = dotenv::var("GUARDED_SECRET").expect("GUARDED_SECRET envar");
    let guarded_val = match req.clone().head().headers.get("x-guarded") {
        Some(val) => val,
        None => return false,
    };

    if guarded_val.to_str().unwrap() != guard_secret {
        return false;
    }

    true
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // get envars
    let cookie_secret = dotenv::var("COOKIE_SECRET").expect("COOKIE_SECRET envar");
    let is_secure = dotenv::var("SECURE_HTTP").expect("SECURE_HTTP envar");
    let port = dotenv::var("PORT").expect("PORT envar");

    let app_state = state::AppState::init_db().await;

    // instanciate http server
    HttpServer::new(move || {
        App::new()
            // cookie middleware
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret.clone().as_bytes())
                    .name("kraken-auth")
                    .secure(is_secure.parse().unwrap()),
            ))
            // state middleware
            .app_data(web::Data::new(app_state.to_owned()))
            .app_data(web::Data::new(state::AppState::init_multipart()))
            .service(controllers::routes::get().guard(guard::fn_guard(protect_routes)))
    })
    // bind to localhost on envar port
    .bind(("127.0.0.1", port.parse().clone().unwrap()))?
    .run()
    .await
}
