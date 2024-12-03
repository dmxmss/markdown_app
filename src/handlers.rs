use rocket::{
    fs::NamedFile, get, http::Status, State 
};
use crate::{
    adapter::DbAdapter,
    parse_file::ParseFile
};

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

// TODO handle TempFile in FromRequest
#[post("/upload", format = "multipart/form-data", data = "<file>")]
pub async fn upload<'r>(file: ParseFile<'r>, adapter: &State<DbAdapter>) -> Result<(), Status> {
    adapter.create_note(
        file.name,
        file.contents
    ).await?;

    Ok(()) 
}
