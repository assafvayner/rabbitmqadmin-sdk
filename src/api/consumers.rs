//! `GET /api/consumers` and related consumer endpoints.

use crate::api::encode_segment;
use crate::types::consumer::Consumer;
use crate::{Client, Result};

impl Client {
    /// `GET /api/consumers` — lists all consumers across all vhosts.
    pub async fn list_consumers(&self) -> Result<Vec<Consumer>> {
        self.get("consumers", None).await
    }

    /// `GET /api/consumers/{vhost}` — lists all consumers within a
    /// single vhost. The vhost is percent-encoded (e.g. `/` becomes
    /// `%2F`).
    pub async fn list_consumers_in_vhost(&self, vhost: &str) -> Result<Vec<Consumer>> {
        self.get(&format!("consumers/{}", encode_segment(vhost)), None)
            .await
    }
}
