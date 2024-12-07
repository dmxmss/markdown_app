use std::{
    fmt,
    error
};
use rocket::http::Status;
use rocket_db_pools::sqlx;

#[derive(Debug)]
pub enum ClientError {
    PayloadTooLarge,
    InvalidUploadFormat
}

impl error::Error for ClientError {}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ClientError::PayloadTooLarge => write!(f, "payload is too large"),
            ClientError::InvalidUploadFormat => write!(f, "payload is too large"),
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    Db(sqlx::Error),
    Io(std::io::Error),
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppError::Db(e) => write!(f, "database error: {e}"),
            AppError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    App(AppError),
    Client(ClientError)
}
impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::App(e) => write!(f, "App error: {e}"),
            Error::Client(e) => write!(f, "Client error: {e}")
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::from(AppError::Io(value))
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::App(_) => Status::InternalServerError,
            Error::Client(ClientError::PayloadTooLarge) => Status::PayloadTooLarge,
            Error::Client(ClientError::InvalidUploadFormat) => Status::BadRequest,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::from(AppError::Db(value))
    }
}

impl From<ClientError> for Error {
    fn from(value: ClientError) -> Self {
        Error::Client(value)
    }
}

impl From<AppError> for Error {
    fn from(value: AppError) -> Self {
        Error::App(value)
    }
}
