//! Monitoring example: connects to a RabbitMQ broker via the Management
//! HTTP API and prints a snapshot of cluster health and the busiest queues.
//!
//! Defaults to `http://localhost:15672` with `guest`/`guest`. Override with
//! the `RABBITMQ_URL`, `RABBITMQ_USER`, and `RABBITMQ_PASS` environment
//! variables:
//!
//! ```sh
//! RABBITMQ_URL=http://broker:15672 RABBITMQ_USER=admin RABBITMQ_PASS=secret \
//!   cargo run --example monitor_queues
//! ```

use rabbitmqadmin_sdk::Client;

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env_or("RABBITMQ_URL", "http://localhost:15672");
    let user = env_or("RABBITMQ_USER", "guest");
    let pass = env_or("RABBITMQ_PASS", "guest");
    let client = Client::new(&url, &user, &pass)?;

    // Identity and cluster name.
    let me = client.whoami().await?;
    println!("logged in as {} (tags: {})", me.name, me.tags);
    println!("cluster: {}", client.cluster_name().await?);

    // Cluster-wide overview.
    let overview = client.overview().await?;
    let ot = &overview.object_totals;
    println!(
        "objects: {} connections, {} channels, {} consumers, {} exchanges, {} queues",
        ot.connections, ot.channels, ot.consumers, ot.exchanges, ot.queues
    );
    let qt = &overview.queue_totals;
    println!(
        "messages: {} total ({} ready, {} unacknowledged)",
        qt.messages, qt.messages_ready, qt.messages_unacknowledged
    );
    if let Some(stats) = &overview.message_stats {
        if let Some(publish) = stats.publish {
            println!("published (lifetime): {publish} messages");
        }
    }

    // Per-node resource usage.
    println!("\nnodes:");
    for node in client.list_nodes().await? {
        println!(
            "  {}: mem_used={} disk_free={} fd_used={}",
            node.name,
            node.mem_used.map_or("-".into(), |v| v.to_string()),
            node.disk_free.map_or("-".into(), |v| v.to_string()),
            node.fd_used.map_or("-".into(), |v| v.to_string()),
        );
    }

    // Top 10 queues by message depth.
    let mut queues = client.list_queues().await?;
    queues.sort_by_key(|q| std::cmp::Reverse(q.messages.unwrap_or(0)));
    println!("\ntop {} queues by message depth:", queues.len().min(10));
    for q in queues.iter().take(10) {
        println!(
            "  {}/{}: messages={} ready={} consumers={}",
            q.vhost,
            q.name,
            q.messages.unwrap_or(0),
            q.messages_ready.unwrap_or(0),
            q.consumers.unwrap_or(0),
        );
    }

    Ok(())
}
