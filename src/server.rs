use rocket::fs::FileServer;

use crate::handlers::*;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Server setup", |rocket| async {
        rocket
            .mount("/", routes![index, upload, upload_form])
            .mount("/public", FileServer::from("static/"))
    })
}
