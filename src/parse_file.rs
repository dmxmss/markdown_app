use rocket::{
    request,
    http::{Status, ContentType},
    Request,
    Data,
    data::{FromData, Outcome, ToByteUnit},
};
use crate::errors::{Error, ClientError};

pub struct ParseFile<'r>{
    pub name: &'r str,
    pub contents: &'r str
}

impl<'r> ParseFile<'r> {
    pub fn new(name: &'r str, contents: &'r str) -> Self {
        ParseFile {
            name,
            contents
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for ParseFile<'r> {
    type Error = Error;
    
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let file_content_type = ContentType::new("multipart", "form-data");
        if req.content_type() != Some(&file_content_type) {
            return Outcome::Forward((data, Status::UnsupportedMediaType));
        }

        // TODO configure rocket, add limits
        let limit = req.limits().get("note").unwrap_or(256.kibibytes());

        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Outcome::Error((Status::PayloadTooLarge, ClientError::PayloadTooLarge.into())),
            Err(e) => return Outcome::Error((Status::InternalServerError, e.into()))
        };

        let string = request::local_cache!(req, string);

        let (filename, contents) = match string.find("&") {
            Some(i) => (&string[..i], &string[(i+1)..]),
            None => return Outcome::Error((Status::UnprocessableEntity, ClientError::InvalidUploadFormat.into()))
        };

        // TODO add UUID's to filenames

        Outcome::Success(ParseFile::new(filename, contents))
    }
}
