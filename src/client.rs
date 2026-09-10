//! Top-level API client

use std::{fmt::Display, sync::Arc};

use getset::CloneGetters;
use parking_lot::RwLock;
use serde_json::json;

use crate::{api_services::{SearchOptions, SearchOptionsBuilder}, types::search::{SOLRQuery, SearchType}};

/// Top-level asynchronous client for OpenLibrary
#[derive(Clone, Debug, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct OpenLibraryClient {
    #[getset(skip)]
    client: Arc<RwLock<reqwest::Client>>,

    /// The app's identifier (see [OpenLibrary User Agents](https://openlibrary.org/developers/api#rate-limits))
    /// First item is the app id
    /// Second item is the email
    app: Option<(String, String)>,
}

#[bon::bon]
impl OpenLibraryClient {
    /// Construct a new [`OpenLibraryClient`]
    #[builder]
    pub fn new(
        /// An existing instance of [`reqwest::ClientBuilder`]
        existing_client: Option<reqwest::ClientBuilder>,

        /// App identification as per [OpenLibrary User Agents](https://openlibrary.org/developers/api#rate-limits)
        #[builder(with = |app_id: impl Display, email: impl Display| (app_id.to_string(), email.to_string())) ]
        app: Option<(String, String)>,
    ) -> Self {
        Self {
            client: Arc::new(RwLock::new(
                existing_client
                    .unwrap_or_default()
                    .user_agent(Self::generate_ua(app.clone()))
                    .cookie_store(true)
                    .build()
                    .expect("Expected client to build successfully"),
            )),
            app,
        }
    }
}

impl OpenLibraryClient {
    fn generate_ua(app: Option<(String, String)>) -> String {
        if let Some((app_id, app_email)) = app {
            format!(
                "{app_id} ({app_email}) open-library-client/{} (https://crates.io/crates/open-library-client)",
                clap::crate_version!()
            )
        } else {
            format!(
                "anonymous-app (unknown@email.com) open-library-client/{} (https://crates.io/crates/open-library-client)",
                clap::crate_version!()
            )
        }
    }

    /// Authenticate this client with a given S3 access token and secret key (see [OpenLibrary API Login](https://openlibrary.org/dev/docs/restful_api#login))
    pub async fn login(
        &self,
        access_key: impl Display,
        secret_key: impl Display,
    ) -> crate::Result<()> {
        let client = self.client.read();
        let result = client
            .post("https://openlibrary.org/account/login")
            .json(&json!({
                "access": &access_key.to_string(),
                "secret": &secret_key.to_string()
            }))
            .send()
            .await?;
        if result.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::invalid_login(
                result.status(),
                result.text().await.unwrap_or(String::from("<unknown>")),
            ))
        }
    }

    /// Return the generated user agent for this client
    pub fn user_agent(&self) -> String {
        Self::generate_ua(self.app())
    }

    /// Get the internal client instance
    pub fn client(&self) -> reqwest::Client {
        self.client.read().clone()
    }

    /// Search for any entity on OpenLibrary
    pub fn search(&self, kind: SearchType, query: impl Into<SOLRQuery>) -> SearchOptionsBuilder {
        SearchOptions::builder(self.clone(), kind, query)
    }
}
