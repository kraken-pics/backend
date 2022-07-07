use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};

use dotenv::dotenv;

use kraken::routes;
use kraken::state::state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // get envars
    let cookie_secret = dotenv::var("COOKIE_SECRET").expect("COOKIE_SECRET must be set");
    let is_secure = dotenv::var("SECURE_HTTP").expect("SECURE_HTTP must be set");
    let port = dotenv::var("PORT").expect("PORT must be set");

    // initialize appstate
    let app_state = state::AppState::init().await;

    // instanciate http server
    HttpServer::new(move || {
        App::new()
            // cookie middleware
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret.as_bytes())
                    .name("kraken-auth")
                    .secure(is_secure.parse().unwrap()),
            ))
            // state middleware
            .app_data(web::Data::new(app_state.clone()))
            // auth routes
            .service(routes::auth::routes::get())
    })
    // bind to localhost on envar port
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}
