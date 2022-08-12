use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::guard::GuardContext;
use actix_web::{guard, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::{web, App};
use dotenv::dotenv;

use kraken::controllers;
use kraken::state;
use kraken::typings::response::ApiResponse;

fn protect_routes(req: &GuardContext) -> bool {
    let guarded: bool = dotenv::var("GUARDED")
        .expect("GUARDED envar")
        .parse()
        .unwrap();
    let guard_secret = dotenv::var("GUARDED_SECRET").expect("GUARDED_SECRET envar");

    // guarded not enabled, early return
    if guarded.eq(&false) {
        return true;
    }

    let guarded_val = match req.clone().head().headers.get("x-guarded") {
        Some(val) => val,
        None => return false,
    };

    if guarded_val.to_str().unwrap() != guard_secret {
        return false;
    }

    true
}

fn handle_errors(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let post_error = ApiResponse {
        success: false,
        message: format!("{}", err),
    };

    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
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
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(handle_errors),
            )
            .service(controllers::routes::get().guard(guard::fn_guard(protect_routes)))
    })
    // bind to localhost on envar port
    .bind(("127.0.0.1", port.parse().clone().unwrap()))?
    .run()
    .await
}
