use actix_web::{web, App, HttpServer};
use dotenv;

use kraken::routes;
use kraken::state::state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = state::AppState::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(routes::auth::routes::get())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
