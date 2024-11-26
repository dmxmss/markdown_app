#[macro_use] extern crate rocket;
#[cfg(test)]
mod tests {
    use markdown_app::{
        fairings::*,
        handlers::*
    };
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn index_ok() {
        let client = Client::tracked(rocket::build().attach(setup())).expect("Server initialization");
        let response = client.get(uri!(index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn index_content_type_is_html() {
        let client = Client::tracked(rocket::build().attach(setup())).expect("Server initialization");
        let response = client.get(uri!(index)).dispatch();

        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }
}
