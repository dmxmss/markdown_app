use rocket::fs::FileServer;

use crate::{
    handlers::*,
    adapter::DbAdapter
};
#[allow(dead_code)]

pub fn base_routes() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Server setup", |rocket| async {
        rocket
            .mount("/", routes![index, upload])
    })
}

pub fn static_server() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Static server", |rocket| async {
        rocket.mount("/public", FileServer::from("static/")) 
    })
}

pub fn db_init() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::try_on_ignite("Database init", |rocket| async {
        match DbAdapter::tokio_postgres("host=localhost user=postgres dbname=markdown_app").await {
            Ok(adapter) => {
                Ok(rocket.manage(adapter))
            },
            Err(e) => {
                eprintln!("{e}");
                Err(rocket)
            }
        }
    })
}
