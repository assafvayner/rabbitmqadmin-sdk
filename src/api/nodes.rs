//! `GET /api/nodes` and `GET /api/nodes/{name}` — node monitoring.

use crate::api::encode_segment;
use crate::types::node::Node;
use crate::{Client, Result};

impl Client {
    /// `GET /api/nodes` — lists all cluster nodes with their resource
    /// usage metrics.
    pub async fn list_nodes(&self) -> Result<Vec<Node>> {
        self.get("nodes", None).await
    }

    /// `GET /api/nodes/{name}` — returns metrics for a single node.
    /// The node name is percent-encoded (e.g. `rabbit@host` becomes
    /// `rabbit%40host`).
    pub async fn get_node(&self, name: &str) -> Result<Node> {
        self.get(&format!("nodes/{}", encode_segment(name)), None)
            .await
    }
}
