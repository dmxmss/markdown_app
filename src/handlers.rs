use rocket::{
    fs::{NamedFile, TempFile}, get, http::Status, State, form::Form
};
use crate::adapter::DbAdapter;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

// TODO handle TempFile in FromRequest
#[post("/upload", format = "multipart/form-data", data = "<upload>")]
pub async fn upload(upload: TempFile<'_>, adapter: &State<DbAdapter>) -> Result<(), Status> {
    if let Err(e) = adapter.create_note(
        upload.name().unwrap_or(""), 
        format!("{upload:?}").as_str()
    ).await {
        eprintln!("{e}");
    }

    Ok(()) 
}
