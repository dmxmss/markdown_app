#[macro_use] extern crate rocket;

use markdown_app::fairings::{base_routes, static_server, db_init};

#[launch]
pub async fn rocket() -> _ {
    rocket::build()
        .attach(base_routes())
        .attach(static_server())
        .attach(db_init())
}
