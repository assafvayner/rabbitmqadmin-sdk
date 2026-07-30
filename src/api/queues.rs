//! `GET /api/queues` and related queue endpoints.

use crate::api::encode_segment;
use crate::types::queue::{
    AckMode, GetMessage, PayloadEncoding, Queue, QueueAction, QueueDeclareOptions,
};
use crate::{Client, Paginated, PaginationQuery, Result};

impl Client {
    /// `GET /api/queues` — lists all queues across all vhosts.
    pub async fn list_queues(&self) -> Result<Vec<Queue>> {
        self.get("queues", None).await
    }

    /// `GET /api/queues` with pagination parameters — returns a
    /// [`Paginated`] page of queues.
    pub async fn list_queues_paged(&self, q: &PaginationQuery) -> Result<Paginated<Queue>> {
        self.get("queues", Some(q)).await
    }

    /// `GET /api/queues/{vhost}` — lists all queues in a single vhost.
    /// The vhost is percent-encoded (e.g. `/` becomes `%2F`).
    pub async fn list_queues_in_vhost(&self, vhost: &str) -> Result<Vec<Queue>> {
        self.get(&format!("queues/{}", encode_segment(vhost)), None)
            .await
    }

    /// `GET /api/queues/{vhost}/{name}` — returns details of a single
    /// queue. Both segments are percent-encoded.
    pub async fn get_queue(&self, vhost: &str, name: &str) -> Result<Queue> {
        self.get_ctx(
            &queue_path(vhost, name),
            None,
            &format!("queue '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `PUT /api/queues/{vhost}/{name}` — declares a queue.
    pub async fn declare_queue(
        &self,
        vhost: &str,
        name: &str,
        opts: &QueueDeclareOptions,
    ) -> Result<()> {
        self.put(&queue_path(vhost, name), opts).await
    }

    /// `DELETE /api/queues/{vhost}/{name}` — deletes a queue.
    pub async fn delete_queue(&self, vhost: &str, name: &str) -> Result<()> {
        self.delete_ctx(
            &queue_path(vhost, name),
            &format!("queue '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `DELETE /api/queues/{vhost}/{name}/contents` — purges all messages
    /// from a queue.
    pub async fn purge_queue(&self, vhost: &str, name: &str) -> Result<()> {
        self.delete_ctx(
            &format!("{}/contents", queue_path(vhost, name)),
            &format!("queue '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `POST /api/queues/{vhost}/{name}/actions` — performs a queue action
    /// (`sync` or `cancel_sync`, for quorum / classic mirrored queues).
    pub async fn queue_action(&self, vhost: &str, name: &str, action: QueueAction) -> Result<()> {
        self.post(
            &format!("{}/actions", queue_path(vhost, name)),
            &serde_json::json!({"action": action.as_str()}),
        )
        .await
    }

    /// `POST /api/queues/{vhost}/{name}/get` — fetches up to `count`
    /// messages from the queue without consuming them client-side.
    ///
    /// Whether the messages are requeued server-side depends on `ack`;
    /// `encoding` controls how payloads are returned (`auto` returns the
    /// payload as a string when it is valid UTF-8, else base64; `base64`
    /// always base64-encodes).
    pub async fn get_messages(
        &self,
        vhost: &str,
        name: &str,
        count: u32,
        ack: AckMode,
        encoding: PayloadEncoding,
    ) -> Result<Vec<GetMessage>> {
        self.post_json(
            &format!("{}/get", queue_path(vhost, name)),
            &serde_json::json!({
                "count": count,
                "ackmode": ack.as_str(),
                "encoding": encoding.as_str(),
            }),
        )
        .await
    }
}

/// Percent-encode a vhost and queue name into a `queues/{vhost}/{name}`
/// relative path segment prefix.
fn queue_path(vhost: &str, name: &str) -> String {
    format!("queues/{}/{}", encode_segment(vhost), encode_segment(name))
}
