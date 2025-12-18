use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("JSON serialization failed")]
    Json(#[from] serde_json::Error),

    #[error("UTF-8 encoding error")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Schema validation failed: {0}")]
    Schema(String),
}

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("Invalid JSON")]
    Json(#[from] serde_json::Error),

    #[error("Schema validation failed: {0}")]
    Schema(String),
}
