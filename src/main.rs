#[macro_use] extern crate rocket;

use markdown_app::{
    server,
    adapter::SqlxPg
};
use rocket_db_pools::Database;

#[launch]
pub async fn rocket() -> _ {
    rocket::build()
        .attach(server::stage())
        .attach(SqlxPg::init())
}
