//! Shared wiremock helpers for integration tests.

use wiremock::matchers::{self, HeaderExactMatcher};
use wiremock::MockServer;

/// Start a fresh mock server for one test.
pub async fn server() -> MockServer {
    MockServer::start().await
}

/// Matcher requiring the HTTP Basic authorization header for
/// `guest:guest` (base64 `Z3Vlc3Q6Z3Vlc3Q=`).
pub fn guest_auth() -> HeaderExactMatcher {
    matchers::header("authorization", "Basic Z3Vlc3Q6Z3Vlc3Q=")
}
