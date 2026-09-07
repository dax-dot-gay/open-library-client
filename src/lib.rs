#![warn(missing_docs)]
//! Asynchronous API client for the [OpenLibrary API](https://openlibrary.org/developers/api) and some parts of its website.

mod error;

pub use error::{OpenLibraryErrorKind, OpenLibraryError as Error, Result as ApiResult};
pub(crate) use error::Result;

pub mod client;
pub use client::OpenLibraryClient;

pub mod api_services;

#[cfg(test)]
mod tests;