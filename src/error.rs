use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Clone, Debug)]
#[error("{0}")]
pub struct StrError(String);

impl From<AppError> for StrError {
    fn from(value: AppError) -> Self {
        Self(value.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppError(String);

impl AppError {
    pub fn from_str<T>(v: T) -> Self
    where
        T: Into<String>,
    {
        Self(v.into())
    }
}

impl<T> From<T> for AppError
where
    T: std::error::Error,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

pub type AppResult<T, E = AppError> = Result<T, E>;
