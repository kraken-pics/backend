//! SeaORM Entity. Generated by sea-orm-codegen 0.9.0

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "urltype")]
pub enum Urltype {
    #[sea_orm(string_value = "DEFAULT")]
    Default,
    #[sea_orm(string_value = "INVISIBLE")]
    Invisible,
    #[sea_orm(string_value = "EMOJI")]
    Emoji,
    #[sea_orm(string_value = "HIRAGANA")]
    Hiragana,
    #[sea_orm(string_value = "FAKE")]
    Fake,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "USER")]
    User,
    #[sea_orm(string_value = "MODERATOR")]
    Moderator,
    #[sea_orm(string_value = "ADMIN")]
    Admin,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "membership")]
pub enum Membership {
    #[sea_orm(string_value = "DEFAULT")]
    Default,
    #[sea_orm(string_value = "PREMIUM")]
    Premium,
}
