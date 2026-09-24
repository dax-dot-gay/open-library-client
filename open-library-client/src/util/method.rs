//! Wrapper around [`reqwest::Method`]

use std::fmt::Display;

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

/// Serializable alternative to [`reqwest::Method`]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
    QUERY,
}

impl From<Method> for reqwest::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::DELETE => reqwest::Method::DELETE,
            Method::HEAD => reqwest::Method::HEAD,
            Method::OPTIONS => reqwest::Method::OPTIONS,
            Method::CONNECT => reqwest::Method::CONNECT,
            Method::PATCH => reqwest::Method::PATCH,
            Method::TRACE => reqwest::Method::TRACE,
            Method::QUERY => reqwest::Method::QUERY,
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.clone() {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::CONNECT => "CONNECT",
            Method::PATCH => "PATCH",
            Method::TRACE => "TRACE",
            Method::QUERY => "QUERY",
        })
    }
}

impl From<Method> for String {
    fn from(value: Method) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for Method {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_case(Case::UpperSnake).trim() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "CONNECT" => Ok(Self::CONNECT),
            "PATCH" => Ok(Self::PATCH),
            "TRACE" => Ok(Self::TRACE),
            "QUERY" => Ok(Self::QUERY),
            other => Err(crate::Error::unknown_http_method(other)),
        }
    }
}

/// Trait to apply to values convertible to a [`Method`]
pub trait IntoMethod {
    /// Convert this value to a [`Method`] if possible
    fn into_method(self) -> crate::Result<Method>;
}

impl<T: Display> IntoMethod for T {
    fn into_method(self) -> crate::Result<Method> {
        Method::try_from(self.to_string())
    }
}
