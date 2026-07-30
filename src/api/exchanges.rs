//! `GET /api/exchanges` and related exchange endpoints.

use crate::api::encode_segment;
use crate::types::exchange::{Exchange, ExchangeDeclareOptions, PublishMessage, PublishResult};
use crate::{Client, Paginated, PaginationQuery, Result};

impl Client {
    /// `GET /api/exchanges` — lists all exchanges across all vhosts.
    pub async fn list_exchanges(&self) -> Result<Vec<Exchange>> {
        self.get("exchanges", None).await
    }

    /// `GET /api/exchanges` with pagination parameters — returns a
    /// [`Paginated`] page of exchanges.
    pub async fn list_exchanges_paged(&self, q: &PaginationQuery) -> Result<Paginated<Exchange>> {
        self.get("exchanges", Some(q)).await
    }

    /// `GET /api/exchanges/{vhost}` — lists all exchanges in a single vhost.
    /// The vhost is percent-encoded (e.g. `/` becomes `%2F`).
    pub async fn list_exchanges_in_vhost(&self, vhost: &str) -> Result<Vec<Exchange>> {
        self.get(&format!("exchanges/{}", encode_segment(vhost)), None)
            .await
    }

    /// `GET /api/exchanges/{vhost}/{name}` — returns details of a single
    /// exchange. Both segments are percent-encoded.
    pub async fn get_exchange(&self, vhost: &str, name: &str) -> Result<Exchange> {
        self.get_ctx(
            &exchange_path(vhost, name),
            None,
            &format!("exchange '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `PUT /api/exchanges/{vhost}/{name}` — declares an exchange.
    pub async fn declare_exchange(
        &self,
        vhost: &str,
        name: &str,
        opts: &ExchangeDeclareOptions,
    ) -> Result<()> {
        self.put(&exchange_path(vhost, name), opts).await
    }

    /// `DELETE /api/exchanges/{vhost}/{name}` — deletes an exchange.
    pub async fn delete_exchange(&self, vhost: &str, name: &str) -> Result<()> {
        self.delete_ctx(
            &exchange_path(vhost, name),
            &format!("exchange '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `POST /api/exchanges/{vhost}/{name}/publish` — publishes a message
    /// to an exchange over the HTTP API. Returns whether the message was
    /// routed to at least one queue.
    pub async fn publish_to_exchange(
        &self,
        vhost: &str,
        name: &str,
        msg: &PublishMessage,
    ) -> Result<PublishResult> {
        self.post_json(&format!("{}/publish", exchange_path(vhost, name)), msg)
            .await
    }

    /// `GET /api/exchanges/{vhost}/{name}/bindings/source` — lists the
    /// bindings for which this exchange is the source.
    ///
    /// Currently returned as raw [`serde_json::Value`]s; this will become
    /// `Vec<Binding>` once the bindings module lands (Task 8 will migrate it).
    pub async fn list_exchange_bindings_source(
        &self,
        vhost: &str,
        name: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.get(
            &format!("{}/bindings/source", exchange_path(vhost, name)),
            None,
        )
        .await
    }
}

/// Percent-encode a vhost and exchange name into a
/// `exchanges/{vhost}/{name}` relative path segment prefix.
fn exchange_path(vhost: &str, name: &str) -> String {
    format!(
        "exchanges/{}/{}",
        encode_segment(vhost),
        encode_segment(name)
    )
}
