use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn get_db() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to connect to db");

    return conn;
}
