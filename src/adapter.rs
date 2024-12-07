use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("markdown_app")]
pub struct SqlxPg(sqlx::PgPool);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Note {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    name: String,
    contents: String
}
