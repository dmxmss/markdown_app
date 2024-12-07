use rocket::{
    fs::{NamedFile, TempFile},
    get, 
    form::Form,
    tokio::fs
};
use rocket_db_pools::{
    Connection,
    sqlx::{self, Row}
};
use crate::{
    adapter::SqlxPg,
    errors::Error
};

type Result<T, E = rocket::response::Debug<Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/upload")]
pub async fn upload_form() -> Option<NamedFile> {
    NamedFile::open("static/form.html").await.ok()
}

// TODO handle TempFile in FromRequest
#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub async fn upload(form: Form<TempFile<'_>>, mut db: Connection<SqlxPg>) -> Result<String> {
    let contents = fs::read_to_string(form.path().unwrap()).await.map_err(Error::from)?;

    let id = sqlx::query(
        "insert into notes (name, contents) values ($1, $2) returning id",
    )
        .bind(form.name().unwrap())
        .bind(contents)
        .fetch_one(&mut **db)
        .await.map_err(Error::from)?;
    
    Ok(id.get::<i32, usize>(0).to_string()) 
}
