#![warn(missing_docs)]

//! Asynchronous API client for the [OpenLibrary API](https://openlibrary.org/developers/api) and some parts of its website.
//! 
//! Offers both a low-level direct API and a high-level wrapper

mod error;
pub use error::{OpenLibraryError, OpenLibraryErrorKind};
pub(crate) use error::{OpenLibraryError as Error, Result};

mod client;
pub use client::Client;

pub mod core;
pub use core::CoreApi;

mod util;
pub(crate) use util::*;

#[cfg(test)]
mod tests;
