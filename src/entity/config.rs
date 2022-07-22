//! SeaORM Entity. Generated by sea-orm-codegen 0.9.0

use super::sea_orm_active_enums::Urltype;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub userid: i32,
    pub embedenabled: i8,
    pub embeddescription: Option<String>,
    pub embedtitle: Option<String>,
    pub embedauthor: Option<String>,
    pub embedauthorurl: Option<String>,
    pub embedsitename: Option<String>,
    pub embedsiteurl: Option<String>,
    pub explodingenabled: i8,
    pub fakeurl: Option<String>,
    pub filepath: Option<String>,
    pub urltype: Urltype,
    pub subdomain: Option<String>,
    pub domain: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Userid",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
