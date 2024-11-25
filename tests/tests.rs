#[macro_use] extern crate rocket;
#[cfg(test)]
mod tests {
    use markdown_app::{
        fairings::*,
        handlers::*
    };
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn index_ok() {
        let client = Client::tracked(rocket::build().attach(setup())).expect("Server initialization");
        let response = client.get(uri!(index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
