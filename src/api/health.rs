//! `GET /api/health/checks/*`, `GET /api/alarms`, and
//! `POST /api/rebalance/queues`.

use crate::api::encode_segment;
use crate::types::health::HealthStatus;
use crate::{Client, Result};

impl Client {
    /// `GET /api/health/checks/alarms` — verifies that no alarms are in
    /// effect in the cluster. When the check FAILS the server responds
    /// with 503 and a JSON body describing the failure; that surfaces as
    /// [`crate::Error::Api`] with `status: 503`, and the error `reason`
    /// carries the failure JSON body — callers should expect that.
    pub async fn health_check_alarms(&self) -> Result<HealthStatus> {
        self.get("health/checks/alarms", None).await
    }

    /// `GET /api/health/checks/local-alarms` — verifies that no alarms
    /// are in effect on the target node. When the check FAILS the server
    /// responds with 503 (with a JSON body); that surfaces as
    /// [`crate::Error::Api`] with `status: 503`, and the error `reason`
    /// carries the failure JSON body — callers should expect that.
    pub async fn health_check_local_alarms(&self) -> Result<HealthStatus> {
        self.get("health/checks/local-alarms", None).await
    }

    /// `GET /api/health/checks/port-listener/{port}` — verifies that a
    /// listener is active on the given port. When the check FAILS the
    /// server responds with 503 (with a JSON body); that surfaces as
    /// [`crate::Error::Api`] with `status: 503`, and the error `reason`
    /// carries the failure JSON body — callers should expect that.
    pub async fn health_check_port_listener(&self, port: u16) -> Result<HealthStatus> {
        self.get(&format!("health/checks/port-listener/{port}"), None)
            .await
    }

    /// `GET /api/health/checks/protocol-listener/{protocol}` — verifies
    /// that a listener for the given protocol (e.g. `\"amqp\"`,
    /// `\"mqtt\"`) is active. The protocol is percent-encoded. When the
    /// check FAILS the server responds with 503 (with a JSON body); that
    /// surfaces as [`crate::Error::Api`] with `status: 503`, and the
    /// error `reason` carries the failure JSON body — callers should
    /// expect that.
    pub async fn health_check_protocol_listener(&self, protocol: &str) -> Result<HealthStatus> {
        self.get(
            &format!(
                "health/checks/protocol-listener/{}",
                encode_segment(protocol)
            ),
            None,
        )
        .await
    }

    /// `GET /api/health/checks/node-is-quorum-critical` — checks whether
    /// the node is quorum-critical (i.e. shutting it down would leave
    /// quorum queues without quorum). When the check FAILS the server
    /// responds with 503 (with a JSON body); that surfaces as
    /// [`crate::Error::Api`] with `status: 503`, and the error `reason`
    /// carries the failure JSON body — callers should expect that.
    pub async fn health_check_node_is_quorum_critical(&self) -> Result<HealthStatus> {
        self.get("health/checks/node-is-quorum-critical", None)
            .await
    }

    /// `GET /api/health/checks/virtual-hosts` — verifies that all
    /// virtual hosts are running on the target node. When the check
    /// FAILS the server responds with 503 (with a JSON body); that
    /// surfaces as [`crate::Error::Api`] with `status: 503`, and the
    /// error `reason` carries the failure JSON body — callers should
    /// expect that.
    pub async fn health_check_virtual_hosts(&self) -> Result<HealthStatus> {
        self.get("health/checks/virtual-hosts", None).await
    }

    /// `GET /api/alarms` — lists the alarms currently in effect in the
    /// cluster, as open-ended alarm descriptors
    /// (e.g. `{"node": ..., "resource": "memory"}`), returned as
    /// [`serde_json::Value`].
    pub async fn list_alarms(&self) -> Result<Vec<serde_json::Value>> {
        self.get("alarms", None).await
    }

    /// `POST /api/rebalance/queues` — asks the cluster to rebalance
    /// queue leadership across nodes.
    pub async fn rebalance_queues(&self) -> Result<()> {
        self.post("rebalance/queues", &serde_json::json!({})).await
    }
}
