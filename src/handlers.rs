use rocket::{
    get,
    fs::NamedFile
};

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}
