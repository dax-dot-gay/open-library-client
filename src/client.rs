//! Common client struct

use getset::CloneGetters;
use serde_json::json;

/// API client instance
#[derive(Clone, Debug, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct Client {
    #[getset(skip)]
    client: reqwest::Client,

    /// The user agent saved for this client
    user_agent: Option<String>,

    /// Whether this client is authenticated
    logged_in: bool,
}

impl Client {
    /// Make a request with this client
    /// Identical functionality to [`reqwest::Client::request`]
    pub fn request(
        &self,
        method: reqwest::Method,
        url: impl reqwest::IntoUrl,
    ) -> reqwest::RequestBuilder {
        self.client.request(method, url)
    }
}

#[bon::bon]
impl Client {
    /// Construct a new [Client] instance
    #[builder]
    pub async fn new(
        #[builder(setters(vis = "", name = authentication_internal))] authentication: Option<(
            String,
            String,
        )>,
        client: Option<reqwest::ClientBuilder>,
        #[builder(into)] user_agent: Option<String>,
    ) -> crate::Result<Client> {
        let mut builder = if let Some(existing) = client {
            existing
        } else {
            reqwest::Client::builder()
        }
        .cookie_store(true);

        if let Some(ua) = user_agent.clone() {
            builder = builder.user_agent(ua);
        }

        let api = builder.build()?;

        let logged_in = if let Some((access, secret)) = authentication {
            let result = api
                .post("https://openlibrary.org/account/login")
                .json(&json!({
                    "access": &access,
                    "secret": &secret
                }))
                .send()
                .await?;
            if result.status().is_success() {
                true
            } else {
                Err(crate::Error::login_failed(
                    result.status().as_u16(),
                    result.text().await.unwrap_or("Unknown error".to_string()),
                ))?;
                unreachable!("Will always error");
            }
        } else {
            false
        };

        Ok(Self {
            client: api,
            user_agent,
            logged_in,
        })
    }
}

impl<S: client_builder::State> ClientBuilder<S> {
    /// Provide login credentials to authenticate this client
    /// For details, see [OpenLibrary's API documentation](https://openlibrary.org/dev/docs/restful_api#login)
    pub fn login(
        self,
        access: impl Into<String>,
        secret: impl Into<String>,
    ) -> ClientBuilder<client_builder::SetAuthentication<S>>
    where
        S::Authentication: client_builder::IsUnset,
    {
        self.authentication_internal((access.into(), secret.into()))
    }
}
