#[macro_use] extern crate rocket;

use markdown_app::fairings::setup;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(setup())
}
