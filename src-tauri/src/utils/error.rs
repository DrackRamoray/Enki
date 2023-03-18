use base64::DecodeError;
use image::error::ImageError;
use reqwest::header::InvalidHeaderName;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqError;
use serde::{ser::Serializer, Serialize};
use serde_json::Error as SerdeError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlError;
use std::io::Error as IOError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("config directory was not found")]
    ConfigDirNotFound,
    #[error(transparent)]
    SerdeError(#[from] SerdeError),
    #[error(transparent)]
    IOError(#[from] IOError),
    #[error(transparent)]
    ReqError(#[from] ReqError),
    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),
    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error(transparent)]
    SqlError(#[from] SqlError),
    #[error(transparent)]
    MigrateError(#[from] MigrateError),
    #[error("database not loaded")]
    DatabaseNotLoaded,
    #[error(transparent)]
    DecodeError(#[from] DecodeError),
    #[error(transparent)]
    ImageError(#[from] ImageError),
    #[error("chat error: {0}")]
    ChatFailed(String),
    #[error("image generate failed: {0}")]
    ImageGenerateFailed(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
