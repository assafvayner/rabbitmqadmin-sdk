//! Error type for the RabbitMQ Management API client.

use thiserror::Error;

/// Errors returned by the RabbitMQ Management API client.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP-level failure (DNS, connection, TLS, timeout, ...).
    #[error("HTTP transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// The server returned a non-2xx status other than 404.
    #[error("RabbitMQ API error {status}: {reason}")]
    Api {
        /// HTTP status code returned by the server.
        status: u16,
        /// Response body / reason returned by the server.
        reason: String,
    },

    /// The server returned 404 for the requested resource.
    #[error("resource not found: {0}")]
    NotFound(String),

    /// The response body could not be deserialized into the expected type.
    ///
    /// The raw `body` is kept for debugging; note it is unbounded in size
    /// for v1 — a future version may truncate it.
    #[error("failed to deserialize response body: {source}")]
    Deserialize {
        /// The underlying serde error.
        source: serde_json::Error,
        /// The raw response body that failed to deserialize.
        body: String,
    },

    /// The base URL or a constructed request URL was invalid.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

/// Result alias using this crate's [`Error`](enum@Error) type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn api_error_displays_status_and_reason() {
        let e = Error::Api {
            status: 401,
            reason: "not authorised".into(),
        };
        assert_eq!(e.to_string(), "RabbitMQ API error 401: not authorised");
    }

    #[test]
    fn not_found_displays_identifier() {
        let e = Error::NotFound("queue 'q1'".into());
        assert_eq!(e.to_string(), "resource not found: queue 'q1'");
    }

    #[test]
    fn invalid_url_displays_url() {
        let e = Error::InvalidUrl("ht!tp://nope".into());
        assert_eq!(e.to_string(), "invalid URL: ht!tp://nope");
    }

    #[test]
    fn deserialize_displays_source_not_body() {
        let source = serde_json::from_str::<String>("42").unwrap_err();
        let e = Error::Deserialize {
            source,
            body: "42".into(),
        };
        let msg = e.to_string();
        assert!(
            msg.starts_with("failed to deserialize response body: "),
            "got: {msg}"
        );
    }
}
