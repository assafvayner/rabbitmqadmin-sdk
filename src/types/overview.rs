//! Response type for `GET /api/overview`.

use serde::Deserialize;
use serde_json::Value;

use super::common::{MessageStats, ObjectTotals, QueueTotals};

/// Cluster overview as returned by `GET /api/overview`.
#[derive(Debug, Clone, Deserialize)]
pub struct Overview {
    /// Version of the management plugin (e.g. `"3.12.4"`).
    #[serde(default)]
    pub management_version: Option<String>,
    /// RabbitMQ server version.
    #[serde(default)]
    pub rabbitmq_version: Option<String>,
    /// Name of the cluster.
    #[serde(default)]
    pub cluster_name: Option<String>,
    /// Erlang/OTP version the nodes are running.
    #[serde(default)]
    pub erlang_version: Option<String>,
    /// Cluster-wide object counts.
    pub object_totals: ObjectTotals,
    /// Cluster-wide message totals.
    pub queue_totals: QueueTotals,
    /// Cluster-wide message rate counters; absent on a fresh cluster.
    #[serde(default)]
    pub message_stats: Option<MessageStats>,
    /// Protocol listeners across the cluster (open-ended entries).
    #[serde(default)]
    pub listeners: Option<Vec<Value>>,
    /// Web contexts served by nodes (open-ended entries).
    #[serde(default)]
    pub contexts: Option<Vec<Value>>,
}
