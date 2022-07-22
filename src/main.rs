use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};

use dotenv::dotenv;

use kraken::controllers;
use kraken::state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // get envars
    let cookie_secret = dotenv::var("COOKIE_SECRET").expect("COOKIE_SECRET envar");
    let is_secure = dotenv::var("SECURE_HTTP").expect("SECURE_HTTP envar");
    let port = dotenv::var("PORT").expect("PORT envar");

    // initialize appstate
    let app_state = state::AppState::init().await;

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
            .app_data(web::Data::new(app_state.clone()))
            // routes
            .service(controllers::routes::get())
    })
    // bind to localhost on envar port
    .bind(("127.0.0.1", port.parse().clone().unwrap()))?
    .run()
    .await
}
