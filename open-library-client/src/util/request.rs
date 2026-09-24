//! Serializable & cacheable request format

use std::{fmt::Display, time::Duration};

use bon::Builder;
use getset::{CloneGetters, WithSetters};
use serde::{Deserialize, Serialize};

use crate::util::caches::Cacheable;

/// A wrapper over [`reqwest::RequestBuilder`] that supports request caching and serialization
#[derive(Serialize, Deserialize, Clone, Debug, Builder, CloneGetters, WithSetters)]
#[builder(start_fn = new)]
#[getset(get_clone = "pub", set_with = "pub")]
pub struct Request {
    /// API endpoint added to `host`
    /// Leading slashes are stripped
    #[builder(into, start_fn)]
    endpoint: String,

    /// Query string
    #[builder(field)]
    query: Vec<(String, String)>,

    /// Request method (defaults to [`crate::util::Method::GET`])
    #[builder(default, with = |x: impl super::IntoMethod| -> Result<_, crate::Error> { Ok(x.into_method()?) })]
    method: super::Method,

    /// Top-level API url (API endpoints added to this)
    /// Trailing slashes are stripped
    #[builder(default = String::from("https://openlibrary.org"), into)]
    host: String,

    /// Timeout duration
    /// Defaults to the client timeout
    timeout: Option<Duration>,
}

impl<S: request_builder::State> RequestBuilder<S> {
    /// Add some serializable data to the query string.
    /// Use [`crate::params`] to easily construct a value for this.
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> crate::Result<Self> {
        let serialized: String = serde_urlencoded::to_string(query)?;
        let new_entries: Vec<(String, String)> = serde_urlencoded::from_str(&serialized)?;
        self.query.extend(new_entries);
        Ok(self)
    }

    /// Add a single value to the query string
    pub fn with_param(mut self, key: impl Into<String>, value: impl Display) -> Self {
        self.query.push((key.into(), value.to_string()));
        self
    }
}

impl Request {
    /// Return the serialized query string of this request
    pub fn query_string(&self) -> String {
        serde_urlencoded::to_string(self.query.clone())
            .expect("Query string input should already be validated")
    }
}
