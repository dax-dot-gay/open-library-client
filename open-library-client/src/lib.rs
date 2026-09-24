#![warn(missing_docs)]

//! Asynchronous API client for the [OpenLibrary API](https://openlibrary.org/developers/api) and some parts of its website.
//! 
//! Offers both a low-level direct API and a high-level wrapper

mod error;
pub use error::{OpenLibraryError, OpenLibraryErrorKind};
pub(crate) use error::{OpenLibraryError as Error, Result};

pub mod core;
pub use core::{OpenLibraryClientCore, types::*};

pub mod util;

pub mod wrapper;

#[cfg(feature = "macros")]
/// A macro that generates and validates SOLR queries from a DSL.
pub use open_library_client_macros::solr;

#[cfg(test)]
mod tests;
