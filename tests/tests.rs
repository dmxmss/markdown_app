#[macro_use] extern crate rocket;
#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::Arc
    };
    use markdown_app::{
        fairings::*,
        handlers::*,
        adapter::{MockPgAdapter, DbPort}
    };
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    fn get_client() -> Client {
        let port = DbPort::new(Arc::new(MockPgAdapter::new()));

        Client::tracked(
            rocket::build()
                .attach(setup())
                .manage(port)
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

    #[test] 
    fn upload_test_note_is_ok() {
        let client = get_client();
        let file = fs::read_to_string("tests/test_note.md").unwrap();

        let req = client.post("/upload")
            .header(ContentType::Plain)
            .body(file);

        assert_eq!(req.dispatch().status(), Status::Ok);
    }
}
