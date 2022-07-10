use sea_orm::DatabaseConnection;

use crate::db::db;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn init() -> Self {
        let result = Self {
            db: db::get_db().await,
        };

        return result;
    }
}
