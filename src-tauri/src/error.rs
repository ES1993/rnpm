pub struct AppError(pub String);

impl<T> From<T> for AppError
where
    T: std::error::Error,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_ref())
    }
}

pub type AppResult<T> = Result<T, AppError>;
