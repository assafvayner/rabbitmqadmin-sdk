//! Types for the `/api/bindings` endpoints.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A binding between an exchange (source) and a queue or exchange
/// (destination), as returned by `GET /api/bindings` and related endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    /// Name of the exchange the binding originates from.
    pub source: String,
    /// Virtual host the binding lives in.
    pub vhost: String,
    /// Name of the queue or exchange messages are routed to.
    pub destination: String,
    /// Type of the destination: `"queue"` or `"exchange"`.
    pub destination_type: String,
    /// Routing key of the binding.
    pub routing_key: String,
    /// Binding arguments (e.g. `"x-match"` for headers exchanges);
    /// open-ended map.
    #[serde(default)]
    pub arguments: Value,
    /// Server-generated key identifying this binding; used in the URL when
    /// deleting an individual binding.
    #[serde(default)]
    pub properties_key: Option<String>,
}
