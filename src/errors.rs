use std::{
    fmt,
    error
};
use rocket::http::Status;

#[derive(Debug)]
pub enum AppError {
    Db(tokio_postgres::error::Error)
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppError::Db(e) => write!(f, "Database error: {e}"),
        }
    }
}

impl From<tokio_postgres::error::Error> for AppError {
    fn from(value: tokio_postgres::error::Error) -> Self {
        AppError::Db(value) 
    }
}

impl From<AppError> for Status {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Db(_) => Status::InternalServerError
        }
    }
}
