use rocket::{
    get,
    fs::{NamedFile, TempFile}, 
    State
};
use crate::adapter::DbPort;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[post("/upload", format = "plain", data = "<upload>")]
pub async fn upload(upload: TempFile<'_>, db_port: &State<DbPort>) -> std::io::Result<()> {
    Ok(()) 
}
