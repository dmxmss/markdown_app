use rocket::fs::FileServer;

use crate::handlers::*;
#[allow(dead_code)]

pub fn setup() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Server setup", |rocket| async {
        rocket
            .mount("/", routes![index])
            .mount("/public", FileServer::from("static/")) 
    })
}
