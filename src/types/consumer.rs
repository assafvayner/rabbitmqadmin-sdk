//! Types for `GET /api/consumers` and related endpoints.

use serde::Deserialize;

/// A consumer attached to a channel, as returned by
/// `GET /api/consumers`.
#[derive(Debug, Clone, Deserialize)]
pub struct Consumer {
    /// Server-assigned or client-provided consumer tag.
    pub consumer_tag: String,
    /// Queue the consumer is attached to (an object with `name` and
    /// `vhost` keys).
    #[serde(default)]
    pub queue: Option<serde_json::Value>,
    /// Details of the channel the consumer is on (open-ended map).
    #[serde(default)]
    pub channel_details: Option<serde_json::Value>,
    /// Whether messages delivered to this consumer require
    /// acknowledgement.
    #[serde(default)]
    pub ack_required: Option<bool>,
    /// Prefetch (QoS) limit applied to this consumer.
    #[serde(default)]
    pub prefetch_count: Option<u32>,
    /// Whether the consumer is actively receiving messages.
    #[serde(default)]
    pub active: Option<bool>,
    /// Consumer arguments (open-ended map).
    #[serde(default)]
    pub arguments: Option<serde_json::Value>,
}
