//! Async Rust SDK for the RabbitMQ Management HTTP API.
//!
//! This crate provides a strongly-typed async client for the RabbitMQ
//! Management Plugin HTTP API (`/api/*`, typically port 15672). It supports
//! monitoring (overview, nodes, queues, exchanges, connections, ...) as well
//! as administrative actions (declaring and deleting queues, exchanges,
//! bindings, vhosts, users, policies, purging queues, and more).
//!
//! # Example
//!
//! ```no_run
//! use rabbitmqadmin_sdk::Client;
//!
//! # async fn example() -> rabbitmqadmin_sdk::Result<()> {
//! let client = Client::new("http://localhost:15672", "guest", "guest")?;
//! let me = client.whoami().await?;
//! println!("logged in as {} (tags: {})", me.name, me.tags);
//! # Ok(())
//! # }
//! ```

mod api;
mod error;
mod pagination;
pub mod types;

pub use error::{Error, Result};
pub use pagination::{Paginated, PaginationQuery};

use api::handle_response;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// The default RabbitMQ vhost, `"/"`.
pub const DEFAULT_VHOST: &str = "/";

/// Async client for the RabbitMQ Management HTTP API.
///
/// Construct with [`Client::new`] or [`Client::builder`]. Cloning a `Client`
/// is cheap (it shares the underlying connection pool).
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    /// Base URL of the API, always ending in `/api/`
    /// (e.g. `http://localhost:15672/api/`).
    base_url: reqwest::Url,
}

/// Builder for [`Client`]. See [`Client::builder`].
pub struct ClientBuilder {
    base_url: String,
    username: String,
    password: String,
    http: Option<reqwest::Client>,
}

impl Client {
    /// Create a [`ClientBuilder`] for the given Management API endpoint and
    /// credentials.
    ///
    /// `base_url` may be given with or without a trailing `/` and with or
    /// without the `/api` path suffix; it is normalized by
    /// [`ClientBuilder::build`].
    pub fn builder(
        base_url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> ClientBuilder {
        ClientBuilder {
            base_url: base_url.into(),
            username: username.into(),
            password: password.into(),
            http: None,
        }
    }

    /// Convenience constructor:
    /// `Client::new("http://localhost:15672", "guest", "guest")`.
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self> {
        Self::builder(base_url, username, password).build()
    }

    /// `GET /api/whoami` — returns the identity the server associates with
    /// this client's credentials.
    pub async fn whoami(&self) -> Result<types::WhoAmI> {
        self.get("whoami", None).await
    }

    /// Low-level `GET` returning a deserialized JSON body.
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&PaginationQuery>,
    ) -> Result<T> {
        let url = self.url(path)?;
        let req = self
            .http
            .get(url)
            .query(&query.map(|q| q.to_pairs()).unwrap_or_default());
        let resp = req.send().await?;
        let body = handle_response(resp).await?;
        serde_json::from_str(&body).map_err(|e| Error::Api {
            status: 0,
            reason: format!("failed to deserialize response: {e}; body: {body}"),
        })
    }

    /// Low-level `PUT` with a JSON body; the response body is ignored.
    // Used by resource modules (queues, exchanges, ...) landing in later
    // milestones; not yet called from within the crate.
    #[allow(dead_code)]
    pub(crate) async fn put<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let resp = self.http.put(self.url(path)?).json(body).send().await?;
        handle_response(resp).await?;
        Ok(())
    }

    /// Low-level `POST` with a JSON body; the response body is ignored.
    #[allow(dead_code)]
    pub(crate) async fn post<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let resp = self.http.post(self.url(path)?).json(body).send().await?;
        handle_response(resp).await?;
        Ok(())
    }

    /// Low-level `POST` with a JSON body, returning a deserialized JSON
    /// response body.
    #[allow(dead_code)]
    pub(crate) async fn post_json<B, T>(&self, path: &str, body: &B) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let resp = self.http.post(self.url(path)?).json(body).send().await?;
        let body = handle_response(resp).await?;
        serde_json::from_str(&body).map_err(|e| Error::Api {
            status: 0,
            reason: format!("failed to deserialize response: {e}; body: {body}"),
        })
    }

    /// Low-level `DELETE`; the response body is ignored.
    #[allow(dead_code)]
    pub(crate) async fn delete(&self, path: &str) -> Result<()> {
        let resp = self.http.delete(self.url(path)?).send().await?;
        handle_response(resp).await?;
        Ok(())
    }

    /// Join a relative API path onto the base URL.
    fn url(&self, path: &str) -> Result<reqwest::Url> {
        self.base_url
            .join(path)
            .map_err(|e| Error::InvalidUrl(format!("{path}: {e}")))
    }
}

impl ClientBuilder {
    /// Use a pre-configured [`reqwest::Client`] (e.g. with custom TLS,
    /// timeouts, or proxies) instead of the default one. The Authorization
    /// and Content-Type default headers are still applied on top of it.
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    /// Normalize the base URL, build the HTTP client with default headers
    /// (HTTP Basic auth and `Content-Type: application/json`), and return
    /// the [`Client`].
    pub fn build(self) -> Result<Client> {
        let trimmed = self.base_url.trim_end_matches('/');
        let with_api = if trimmed.ends_with("/api") {
            format!("{trimmed}/")
        } else {
            format!("{trimmed}/api/")
        };
        let base_url = reqwest::Url::parse(&with_api)
            .map_err(|e| Error::InvalidUrl(format!("{}: {e}", self.base_url)))?;

        let mut headers = HeaderMap::new();
        let auth = format!("{}:{}", self.username, self.password);
        let mut auth_value =
            HeaderValue::from_str(&format!("Basic {}", base64_encode(auth.as_bytes())))
                .map_err(|e| Error::InvalidUrl(format!("invalid credentials: {e}")))?;
        auth_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_value);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = match self.http {
            Some(c) => c,
            None => reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        };

        Ok(Client { http, base_url })
    }
}

/// Minimal base64 (standard alphabet, with padding) encoder, to avoid an
/// extra dependency just for the Authorization header.
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]);
        out.push(ALPHABET[(n >> 18) as usize & 0x3f] as char);
        out.push(ALPHABET[(n >> 12) as usize & 0x3f] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(n >> 6) as usize & 0x3f] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[n as usize & 0x3f] as char
        } else {
            '='
        });
    }
    out
}
