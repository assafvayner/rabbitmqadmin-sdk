//! `GET /api/bindings` and related binding endpoints.

use crate::api::encode_segment;
use crate::types::binding::Binding;
use crate::{Client, Paginated, PaginationQuery, Result};

/// Request body for `POST /api/bindings/{vhost}/e/{exchange}/q/{queue}`.
#[derive(Debug, Clone, serde::Serialize)]
struct BindRequest {
    /// Routing key of the new binding.
    routing_key: String,
    /// Optional binding arguments (e.g. `"x-match"` for headers exchanges).
    arguments: serde_json::Value,
}

impl Client {
    /// `GET /api/bindings` — lists all bindings across all vhosts.
    pub async fn list_bindings(&self) -> Result<Vec<Binding>> {
        self.get("bindings", None).await
    }

    /// `GET /api/bindings` with pagination parameters — returns a
    /// [`Paginated`] page of bindings.
    pub async fn list_bindings_paged(&self, q: &PaginationQuery) -> Result<Paginated<Binding>> {
        self.get("bindings", Some(q)).await
    }

    /// `GET /api/bindings/{vhost}` — lists all bindings in a single vhost.
    /// The vhost is percent-encoded (e.g. `/` becomes `%2F`).
    pub async fn list_bindings_in_vhost(&self, vhost: &str) -> Result<Vec<Binding>> {
        self.get(&format!("bindings/{}", encode_segment(vhost)), None)
            .await
    }

    /// `GET /api/queues/{vhost}/{queue}/bindings` — lists all bindings of a
    /// single queue. Both segments are percent-encoded.
    pub async fn list_queue_bindings(&self, vhost: &str, queue: &str) -> Result<Vec<Binding>> {
        self.get(
            &format!(
                "queues/{}/{}/bindings",
                encode_segment(vhost),
                encode_segment(queue)
            ),
            None,
        )
        .await
    }

    /// `GET /api/bindings/{vhost}/e/{exchange}/q/{queue}` — lists all
    /// bindings between a single exchange and a single queue. All segments
    /// are percent-encoded.
    pub async fn list_bindings_between(
        &self,
        vhost: &str,
        exchange: &str,
        queue: &str,
    ) -> Result<Vec<Binding>> {
        self.get(&binding_path(vhost, exchange, queue), None).await
    }

    /// `POST /api/bindings/{vhost}/e/{exchange}/q/{queue}` — creates a
    /// binding from `exchange` to `queue` with the given routing key and
    /// arguments.
    pub async fn bind(
        &self,
        vhost: &str,
        exchange: &str,
        queue: &str,
        routing_key: &str,
        arguments: serde_json::Value,
    ) -> Result<()> {
        self.post(
            &binding_path(vhost, exchange, queue),
            &BindRequest {
                routing_key: routing_key.to_owned(),
                arguments,
            },
        )
        .await
    }

    /// `DELETE /api/bindings/{vhost}/e/{exchange}/q/{queue}/{properties_key}`
    /// — deletes an individual binding identified by its server-generated
    /// `properties_key`.
    pub async fn unbind(
        &self,
        vhost: &str,
        exchange: &str,
        queue: &str,
        properties_key: &str,
    ) -> Result<()> {
        self.delete_ctx(
            &format!(
                "{}/{}",
                binding_path(vhost, exchange, queue),
                encode_segment(properties_key)
            ),
            &format!(
                "binding '{properties_key}' from exchange '{exchange}' to queue '{queue}' in vhost '{vhost}'"
            ),
        )
        .await
    }
}

/// Percent-encode vhost, exchange, and queue into a
/// `bindings/{vhost}/e/{exchange}/q/{queue}` relative path segment prefix.
fn binding_path(vhost: &str, exchange: &str, queue: &str) -> String {
    format!(
        "bindings/{}/e/{}/q/{}",
        encode_segment(vhost),
        encode_segment(exchange),
        encode_segment(queue)
    )
}
