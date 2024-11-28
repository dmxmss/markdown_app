#[macro_use] extern crate rocket;

use std::sync::Arc;
use markdown_app::fairings::setup;
use markdown_app::adapter::{PgAdapter, DbPort};

#[launch]
pub fn rocket() -> _ {
    let port = DbPort::new(Arc::new(PgAdapter::new()));

    rocket::build()
        .attach(setup())
        .manage(port)
}
