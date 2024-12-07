#[macro_use] extern crate rocket;
#[cfg(test)]
mod tests {
    use std::fs;

    use figment::providers::{Format, Toml};
    use figment::Figment;
    use markdown_app::{
        server,
        handlers::*,
        adapter::SqlxPg
    };

    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    fn get_client() -> Client {
        Client::tracked(
            rocket::build()
                .attach(server::stage())
        ).expect("Client init")
    }

    #[test]
    fn index_ok() {
        let client = get_client();
        let response = client.get(uri!(index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn index_content_type_is_html() {
        let client = get_client();
        let response = client.get(uri!(index)).dispatch();

        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }
}
