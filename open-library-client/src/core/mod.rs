//! Low-level API bindings

pub mod types;

use bytes::Bytes;
use reqwest::{Method, StatusCode};

use crate::{
    core::types::{GenericSearchBuilder, generic_search},
    params,
};

/// Simple client to wrap low-level API calls
#[derive(Clone, Debug)]
pub struct OpenLibraryClientCore(reqwest::Client);

impl OpenLibraryClientCore {
    /// Create a new client with an optional user agent
    pub fn new(user_agent: Option<impl Into<String>>) -> Self {
        let mut builder = reqwest::Client::builder().cookie_store(true);
        if let Some(ua) = user_agent.map(Into::into) {
            builder = builder.user_agent(ua);
        }

        Self(builder.build().unwrap())
    }

    /// Create a client from an existing [`reqwest::Client`]
    pub fn from_existing(client: reqwest::ClientBuilder, user_agent: Option<impl Into<String>>) -> crate::Result<Self> {
        let mut builder = client.cookie_store(true);
        if let Some(ua) = user_agent.map(Into::into) {
            builder = builder.user_agent(ua);
        }

        Ok(Self(builder.build()?))
    }

    /// Generate a request builder prefixed with "https://openlibrary.org/". Leading slashes are trimmed.
    pub fn request(&self, method: Method, endpoint: impl Into<String>) -> reqwest::RequestBuilder {
        self.0.request(method, format!("https://openlibrary.org/{}", endpoint.into().trim_start_matches('/')))
    }

    /// Make a request with this client, with no prefix
    /// Identical functionality to [`reqwest::Client::request`]
    pub fn request_raw(
        &self,
        method: Method,
        url: impl reqwest::IntoUrl
    ) -> reqwest::RequestBuilder {
        self.0.request(method, url)
    }
}

impl OpenLibraryClientCore {
    /// Search for books
    /// Endpoint: [/search.json](https://openlibrary.org/search.json)
    pub fn search_books(
        &self,
        query: impl Into<String>,
    ) -> GenericSearchBuilder<types::SearchWork> {
        generic_search(
            self.clone(),
            types::SearchResultKind::Work,
            query.into(),
        )
    }

    /// Search for authors
    /// Endpoint: [/search/authors.json](https://openlibrary.org/search/authors.json)
    pub fn search_authors(
        &self,
        query: impl Into<String>,
    ) -> GenericSearchBuilder<types::SearchAuthor> {
        generic_search(
            self.clone(),
            types::SearchResultKind::Author,
            query.into(),
        )
    }

    /// Search for subject
    /// Endpoint: [/search/subjects.json](https://openlibrary.org/search/subjects.json)
    pub fn search_subjects(
        &self,
        query: impl Into<String>,
    ) -> GenericSearchBuilder<types::SearchSubject> {
        generic_search(
            self.clone(),
            types::SearchResultKind::Subject,
            query.into(),
        )
    }

    /// Retrieve a specific author
    /// Endpoint: [/authors/{author}.json](https://openlibrary.org/authors/{author}.json)
    pub async fn get_author(
        &self,
        olid: impl Into<String>,
    ) -> crate::Result<Option<types::SelectedAuthor>> {
        let result = self
            .request(Method::GET, format!("authors/{}.json", olid.into()))
            .send()
            .await?;
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
    pub async fn get_work(
        &self,
        olid: impl Into<String>,
    ) -> crate::Result<Option<types::SelectedWork>> {
        let result = self
            .request(Method::GET, format!("works/{}.json", olid.into()))
            .send()
            .await?;
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
    pub async fn get_edition(
        &self,
        olid: impl Into<String>,
    ) -> crate::Result<Option<types::SelectedEdition>> {
        let result = self
            .request(Method::GET, format!("books/{}.json", olid.into()))
            .send()
            .await?;
        if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else if result.status().is_success() {
            let val = result.json::<serde_json::Value>().await?;
            //println!("{val:#?}");
            Ok(Some(serde_json::from_value::<types::SelectedEdition>(val)?))
        } else {
            result.error_for_status()?;
            unreachable!();
        }
    }
}

#[bon::bon]
impl OpenLibraryClientCore {
    /// Retrieve an author's works
    /// Endpoint: [/authors/{olid}/works.json](https://openlibrary.org/authors/{olid}/works.json)
    #[builder(finish_fn = get)]
    pub async fn get_author_works(
        &self,
        #[builder(start_fn, into)] author: String,

        /// Maximum number of results to return
        limit: Option<u64>,

        /// How many results to offset by
        offset: Option<u64>,
    ) -> crate::Result<Option<types::PageLinkedRecords<types::SelectedWork>>> {
        let result = self
            .request(Method::GET, format!("authors/{}/works.json", author))
            .query(params!("limit" = limit, "offset" = offset))
            .send()
            .await?;
        if result.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else if result.status().is_success() {
            let val = result.json::<serde_json::Value>().await?;
            //println!("{val:#?}");
            Ok(Some(serde_json::from_value::<
                types::PageLinkedRecords<types::SelectedWork>,
            >(val)?))
        } else {
            result.error_for_status()?;
            unreachable!();
        }
    }

    /// Return details about a specified subject
    /// Endpoint: [/subjects/{subject}.json](https://openlibrary.org/subjects/{subject}.json)
    #[builder(finish_fn = get)]
    pub async fn get_subject(
        &self,

        #[builder(start_fn, into)] subject: String,

        /// Whether to return details (will fill out relation fields)
        #[builder(default = false)]
        details: bool,

        /// Only return works that are ebooks
        #[builder(default = false)]
        ebooks: bool,

        /// Set a published range for works
        #[builder(with = |low: i64, high: i64| format!("{low}-{high}"))]
        published_in: Option<String>,

        /// Maximum number of results to return
        limit: Option<u64>,

        /// How many results to offset by
        offset: Option<u64>,
    ) -> crate::Result<types::SelectedSubject> {
        let result = self.request(Method::GET, format!("subjects/{subject}.json")).query(params!(
            "details" = details,
            "ebooks" = ebooks,
            "published_in" = published_in,
            "limit" = limit,
            "offset" = offset
        )).send().await?.error_for_status()?;
        let val = result.json::<serde_json::Value>().await?;
        //println!("{val:#?}");
        Ok(serde_json::from_value::<types::SelectedSubject>(val)?)
    }

    /// Get the URL to an image (cover or author)
    /// Endpoint: [/{type}/{key}/{value}-{size}.jpg](https://openlibrary.org/{type}/{key}/{value}-{size}.jpg)
    #[builder(finish_fn = get)]
    pub fn get_cover_image_url(
        &self,
        #[builder(start_fn, into)] key: String,

        /// Whether this cover is a book or an author
        #[builder(default)] cover_type: types::covers::CoverType,

        /// What to key by
        #[builder(default)] key_type: types::covers::CoverKeyType,

        /// What size image to return
        #[builder(default)] size: types::covers::CoverImageSize,
    ) -> String {
        format!("https://covers.openlibrary.org/{cover_type}/{key_type}/{key}-{size}.jpg")
    }

    /// Return the byte data of an image (cover or author)
    /// Endpoint: [/{type}/{key}/{value}-{size}.jpg](https://openlibrary.org/{type}/{key}/{value}-{size}.jpg)
    #[builder(finish_fn = get)]
    pub async fn get_cover_image(
        &self,
        #[builder(start_fn, into)] key: String,

        /// Whether this cover is a book or an author
        #[builder(default)] cover_type: types::covers::CoverType,

        /// What to key by
        #[builder(default)] key_type: types::covers::CoverKeyType,

        /// What size image to return
        #[builder(default)] size: types::covers::CoverImageSize,
    ) -> crate::Result<Bytes> {
        let url = self.get_cover_image_url(key).cover_type(cover_type).key_type(key_type).size(size).get();
        let response = self.request_raw(Method::GET, url).send().await?.error_for_status()?;
        Ok(response.bytes().await?)
    }

    /// Return the metadata of an image (cover or author)
    /// Endpoint: [/{type}/{key}/{value}.json](https://openlibrary.org/{type}/{key}/{value}.json)
    #[builder(finish_fn = get)]
    pub async fn get_cover_image_data(
        &self,
        #[builder(start_fn, into)] key: String,

        /// Whether this cover is a book or an author
        #[builder(default)] cover_type: types::covers::CoverType,

        /// What to key by
        #[builder(default)] key_type: types::covers::CoverKeyType
    ) -> crate::Result<types::CoverImage> {
        let url = format!("https://covers.openlibrary.org/{cover_type}/{key_type}/{key}.json");
        let response = self.request_raw(Method::GET, url).send().await?.error_for_status()?;
        Ok(response.json::<types::CoverImage>().await?)
    }
}
