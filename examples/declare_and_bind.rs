//! Admin workflow example: declares an exchange and a queue, binds them,
//! publishes a message over the HTTP API, verifies it landed, then cleans
//! everything up.
//!
//! Defaults to `http://localhost:15672` with `guest`/`guest`. Override with
//! the `RABBITMQ_URL`, `RABBITMQ_USER`, and `RABBITMQ_PASS` environment
//! variables:
//!
//! ```sh
//! RABBITMQ_URL=http://broker:15672 RABBITMQ_USER=admin RABBITMQ_PASS=secret \
//!   cargo run --example declare_and_bind
//! ```

use rabbitmqadmin_sdk::types::exchange::{ExchangeDeclareOptions, PublishMessage};
use rabbitmqadmin_sdk::types::queue::QueueDeclareOptions;
use rabbitmqadmin_sdk::{Client, DEFAULT_VHOST};

const EXCHANGE: &str = "sdk.example.ex";
const QUEUE: &str = "sdk.example.q";
const ROUTING_KEY: &str = "demo";

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env_or("RABBITMQ_URL", "http://localhost:15672");
    let user = env_or("RABBITMQ_USER", "guest");
    let pass = env_or("RABBITMQ_PASS", "guest");
    let client = Client::new(&url, &user, &pass)?;
    let vhost = DEFAULT_VHOST;

    println!("declaring fanout exchange '{EXCHANGE}' in vhost '{vhost}'...");
    client
        .declare_exchange(
            vhost,
            EXCHANGE,
            &ExchangeDeclareOptions {
                type_: "fanout".into(),
                durable: true,
                auto_delete: false,
                internal: false,
                arguments: Default::default(),
            },
        )
        .await?;

    println!("declaring durable queue '{QUEUE}'...");
    client
        .declare_queue(
            vhost,
            QUEUE,
            &QueueDeclareOptions {
                durable: true,
                auto_delete: false,
                arguments: Default::default(),
            },
        )
        .await?;

    println!("binding '{QUEUE}' to '{EXCHANGE}' with routing key '{ROUTING_KEY}'...");
    client
        .bind(vhost, EXCHANGE, QUEUE, ROUTING_KEY, serde_json::json!({}))
        .await?;

    println!("publishing a message...");
    let result = client
        .publish_to_exchange(
            vhost,
            EXCHANGE,
            &PublishMessage::new(ROUTING_KEY, "hello from rabbitmqadmin-sdk"),
        )
        .await?;
    println!("  routed: {}", result.routed);

    let q = client.get_queue(vhost, QUEUE).await?;
    println!(
        "queue '{QUEUE}' now holds {} message(s)",
        q.messages.unwrap_or(0)
    );

    println!("purging queue '{QUEUE}'...");
    client.purge_queue(vhost, QUEUE).await?;

    // Cleanup. (Unbinding is skipped here: deleting a binding requires the
    // server-generated `properties_key`, which `bind` does not return —
    // deleting the queue removes all of its bindings anyway.)
    println!("deleting queue '{QUEUE}'...");
    client.delete_queue(vhost, QUEUE).await?;
    println!("deleting exchange '{EXCHANGE}'...");
    client.delete_exchange(vhost, EXCHANGE).await?;

    println!("done.");
    Ok(())
}
