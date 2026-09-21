//! Low-level API bindings

pub mod types;

use reqwest::{Method, StatusCode};

use crate::core::types::{GenericSearchBuilder, generic_search};

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
    ) -> GenericSearchBuilder<types::SearchWork> {
        generic_search(self.client.clone(), types::SearchResultKind::Work, query.into())
    }

    /// Search for authors
    /// Endpoint: [/search/authors.json](https://openlibrary.org/search/authors.json)
    pub fn search_authors(
        &self,
        query: impl Into<String>
    ) -> GenericSearchBuilder<types::SearchAuthor> {
        generic_search(self.client.clone(), types::SearchResultKind::Author, query.into())
    }

    /// Search for subject
    /// Endpoint: [/search/subjects.json](https://openlibrary.org/search/subjects.json)
    pub fn search_subjects(
        &self,
        query: impl Into<String>
    ) -> GenericSearchBuilder<types::SearchSubject> {
        generic_search(self.client.clone(), types::SearchResultKind::Subject, query.into())
    }

    /// Retrieve a specific author
    /// Endpoint: [/authors/{author}.json](https://openlibrary.org/authors/{author}.json)
    pub async fn get_author(&self, olid: impl Into<String>) -> crate::Result<Option<types::SelectedAuthor>> {
        let result = self.client.request(Method::GET, format!("authors/{}.json", olid.into())).send().await?;
        if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else if result.status().is_success() {
            let val = result.json::<serde_json::Value>().await?;
            //println!("{val}");
            Ok(Some(serde_json::from_value::<types::SelectedAuthor>(val)?))
        } else {
            result.error_for_status()?;
            unreachable!();
        }
    }

    /// Retrieve a specific work
    /// Endpoint: [/works/{work}.json](https://openlibrary.org/works/{work}.json)
    pub async fn get_work(&self, olid: impl Into<String>) -> crate::Result<Option<types::SelectedWork>> {
        let result = self.client.request(Method::GET, format!("works/{}.json", olid.into())).send().await?;
        if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else if result.status().is_success() {
            let val = result.json::<serde_json::Value>().await?;
            //println!("{val:#?}");
            Ok(Some(serde_json::from_value::<types::SelectedWork>(val)?))
        } else {
            result.error_for_status()?;
            unreachable!();
        }
    }

    /// Retrieve a specific edition
    /// Endpoint: [/books/{book}.json](https://openlibrary.org/books/{book}.json)
    pub async fn get_edition(&self, olid: impl Into<String>) -> crate::Result<Option<types::SelectedEdition>> {
        let result = self.client.request(Method::GET, format!("books/{}.json", olid.into())).send().await?;
        if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else if result.status().is_success() {
            let val = result.json::<serde_json::Value>().await?;
            //println!("{val}");
            Ok(Some(serde_json::from_value::<types::SelectedEdition>(val)?))
        } else {
            result.error_for_status()?;
            unreachable!();
        }
    }
    
}
