//! Types for the `/api/health/checks/*` endpoints.

use serde::Deserialize;

/// Result of a health check, as returned by `GET /api/health/checks/*`.
///
/// A passing check responds with HTTP 200 and `{"status": "ok"}`. A
/// failing check responds with HTTP 503 and a JSON body describing the
/// failure; the client surfaces that as [`crate::Error::Api`], so this
/// struct is only ever produced for passing checks.
#[derive(Debug, Clone, Deserialize)]
pub struct HealthStatus {
    /// Check outcome, e.g. `\"ok\"`.
    pub status: String,
    /// Human-readable failure reason (present on failures).
    #[serde(default)]
    pub reason: Option<String>,
    /// Alarm descriptors relevant to the check (present on failures).
    #[serde(default)]
    pub alarms: Option<Vec<serde_json::Value>>,
}
