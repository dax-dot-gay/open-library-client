//! Custom error implementation
use anyhow::anyhow;
use reqwest::StatusCode;

/// Custom error type for this crate
#[derive(Debug, thiserror::Error, thiserror_ext::Construct, thiserror_ext::Arc)]
#[thiserror_ext(newtype(name = OpenLibraryError))]
pub enum OpenLibraryErrorKind {
    /// An unhandled error
    #[error("Unhandled error: {0:?}")]
    #[construct(skip)]
    Unhandled(#[from] anyhow::Error),

    /// Wrapper around [`reqwest::Error`]
    #[error("HTTP request/response error: {0:?}")]
    #[construct(skip)]
    Reqwest(#[from] reqwest::Error),

    /// Wrapper around [`serde_json::Error`]
    #[error("JSON error: {0:?}")]
    #[construct(skip)]
    SerdeJson(#[from] serde_json::Error),

    /// Error on invalid login credentials
    #[error("Invalid login: returned code {code} with reason {reason}")]
    InvalidLogin {
        /// HTTP status code
        code: StatusCode,

        /// Reason returned from the server
        reason: String,
    },
}

impl OpenLibraryError {
    fn wrap(error: OpenLibraryErrorKind) -> Self {
        Self::from(error)
    }

    /// Generates an unhandled error type with [`anyhow::Error`]
    pub fn unhandled(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::wrap(OpenLibraryErrorKind::Unhandled(anyhow!(error)))
    }
}

/// Result wrapper
pub type Result<T> = std::result::Result<T, OpenLibraryError>;
