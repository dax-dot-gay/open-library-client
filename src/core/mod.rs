//! Low-level API bindings

pub mod types;

use crate::{core::types::{GenericSearchBuilder, generic_search}};

/// Struct representing the core API wrapper
#[derive(Clone, Debug)]
pub struct CoreApiHandler {
    pub(crate) client: crate::Client,
}

/// Trait for fetching a [`CoreApiHandler`] instance
pub trait CoreApi {
    /// Return an instance of [`CoreApiHandler`] bound to this client
    fn core(&self) -> CoreApiHandler;
}

impl CoreApi for crate::Client {
    fn core(&self) -> CoreApiHandler {
        CoreApiHandler {
            client: self.clone(),
        }
    }
}

impl CoreApiHandler {
    /// Search for books
    /// Endpoint: [/search.json](https://openlibrary.org/search.json)
    pub fn search_books(
        &self,
        query: impl Into<String>
    ) -> GenericSearchBuilder {
        generic_search(self.client.clone(), types::EntityType::Work, query.into())
    }

    /// Search for authors
    /// Endpoint: [/search/authors.json](https://openlibrary.org/search/authors.json)
    pub fn search_authors(
        &self,
        query: impl Into<String>
    ) -> GenericSearchBuilder {
        generic_search(self.client.clone(), types::EntityType::Author, query.into())
    }

    /// Search for subject
    /// Endpoint: [/search/subjects.json](https://openlibrary.org/search/subjects.json)
    pub fn search_subjects(
        &self,
        query: impl Into<String>
    ) -> GenericSearchBuilder {
        generic_search(self.client.clone(), types::EntityType::Subject, query.into())
    }
}
