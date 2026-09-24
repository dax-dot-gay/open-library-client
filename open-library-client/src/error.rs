//! Custom error implementation
use anyhow::anyhow;

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

    /// Wrapper around [`std::io::Error`]
    #[error("I/O error: {0:?}")]
    #[construct(skip)]
    Io(#[from] std::io::Error),

    /// Wrapper around [`serde_json::Error`]
    #[error("JSON error: {0:?}")]
    #[construct(skip)]
    SerdeJson(#[from] serde_json::Error),

    /// Wrapper around [`serde_urlencoded::de::Error`]
    #[error("URL deserialization error: {0:?}")]
    #[construct(skip)]
    UrlDecode(#[from] serde_urlencoded::de::Error),

    /// Wrapper around [`serde_urlencoded::ser::Error`]
    #[error("URL serialization error: {0:?}")]
    #[construct(skip)]
    UrlEncode(#[from] serde_urlencoded::ser::Error),

    /// Wrapper around [`cached::stores::BuildError`]
    #[error("Failed to construct cache: {0:?}")]
    #[construct(skip)]
    CacheBuild(#[from] cached::stores::BuildError),

    /// Invalid authentication provided
    #[error("Authentication failed with code {code}: {reason}")]
    LoginFailed {
        /// Error status code
        code: u16,

        /// Failure reason
        reason: String
    },

    /// Invalid type in /{key}/{value} string
    #[error("Invalid type {given} in keystring, expected {expected}")]
    KeyType {
        /// Provided type
        given: String,

        /// Expected type
        expected: String
    },

    /// Cache with the specified name already exists
    #[error("A cache named {name} already exists.")]
    CacheExists {
        /// Name of the cache
        name: String
    },

    /// Unable to convert the supplied value into a Method
    #[error("Unknown method: {method}")]
    UnknownHttpMethod {
        /// Supplied method
        method: String
    }
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
