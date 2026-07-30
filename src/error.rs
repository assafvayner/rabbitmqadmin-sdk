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

    /// The base URL or a constructed request URL was invalid.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

/// Result alias using this crate's [`Error`] type.
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
}
