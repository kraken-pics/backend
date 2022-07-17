use crate::entity::sea_orm_active_enums::{Membership, Role};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct Stats {
    pub users: usize,
    pub uploads: usize,
    pub storage: u64,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub message: String,
    pub statistics: Stats,
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub role: Role,
    pub membership: Membership,
    pub uploadtoken: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<User>,
}
