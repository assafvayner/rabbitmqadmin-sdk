//! `GET /api/channels` and related channel endpoints.

use crate::api::encode_segment;
use crate::types::channel::Channel;
use crate::{Client, Result};

impl Client {
    /// `GET /api/channels` — lists all channels across all connections.
    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        self.get("channels", None).await
    }

    /// `GET /api/channels/{name}` — returns a single channel. The name
    /// is percent-encoded (channel names look like
    /// `"127.0.0.1:52341 -> 127.0.0.1:5672 (1)"`).
    pub async fn get_channel(&self, name: &str) -> Result<Channel> {
        self.get_ctx(
            &format!("channels/{}", encode_segment(name)),
            None,
            &format!("channel '{name}'"),
        )
        .await
    }
}
