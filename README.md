# rabbitmqadmin-sdk

An async Rust SDK for the [RabbitMQ Management HTTP API](https://www.rabbitmq.com/docs/management).
It provides a strongly-typed client built on `reqwest` + `serde` covering monitoring, administration,
and day-2 operations for RabbitMQ 3.12+ / 4.x brokers with the management plugin enabled
(typically on port 15672).

## Features

- **Monitoring**: cluster overview, nodes (memory/disk/fd metrics), queues, exchanges, connections,
  channels, consumers, message-rate stats.
- **Administration**: declare and delete queues, exchanges, bindings, vhosts, users, permissions,
  policies, operator policies, and runtime parameters.
- **Operations**: purge queues, publish and fetch messages over HTTP, close connections
  (with `X-Reason`), export/import definitions, six health checks, list alarms, and trigger
  queue-leadership rebalancing.
- **Ergonomics**: percent-encoding of vhost/queue/exchange segments handled for you, contextual
  `NotFound` errors, server-side pagination support, `no_std`-free async via tokio/reqwest.

## Quickstart

Add the dependency and point a `Client` at your broker:

```toml
[dependencies]
rabbitmqadmin-sdk = "0.0.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust,no_run
use rabbitmqadmin_sdk::Client;

# async fn example() -> rabbitmqadmin_sdk::Result<()> {
let client = Client::new("http://localhost:15672", "guest", "guest")?;
let me = client.whoami().await?;
println!("logged in as {} (tags: {})", me.name, me.tags.join(","));

for q in client.list_queues().await? {
    println!("{}/{}: {} messages", q.vhost, q.name, q.messages.unwrap_or(0));
}
# Ok(())
# }
```

Use `Client::builder(...)` to supply a pre-configured `reqwest::Client` (custom TLS, timeouts,
proxies) via `.http_client(...)`.

## Endpoint coverage

| Area | Methods | Endpoints |
|---|---|---|
| Auth | `whoami` | `GET /api/whoami` |
| Overview | `overview`, `cluster_name` | `GET /api/overview`, `GET|PUT /api/cluster-name` (read) |
| Nodes | `list_nodes`, `get_node` | `GET /api/nodes[/{name}]` |
| Queues | `list_queues` (+`_paged`, `_in_vhost`), `get_queue`, `declare_queue`, `delete_queue`, `purge_queue`, `queue_action`, `get_messages` | `GET|PUT|DELETE /api/queues/...`, `POST .../actions`, `POST .../get` |
| Exchanges | `list_exchanges` (+`_paged`, `_in_vhost`), `get_exchange`, `declare_exchange`, `delete_exchange`, `publish_to_exchange`, `list_exchange_bindings_source` | `GET|PUT|DELETE /api/exchanges/...`, `POST .../publish` |
| Bindings | `list_bindings` (+`_paged`, `_in_vhost`), `list_queue_bindings`, `list_bindings_between`, `bind`, `unbind` | `GET|POST|DELETE /api/bindings/...` |
| Vhosts | `list_vhosts`, `get_vhost`, `create_vhost`, `delete_vhost`, `list_vhost_permissions` | `GET|PUT|DELETE /api/vhosts[/{name}]`, `GET .../permissions` |
| Users | `list_users`, `get_user`, `create_user`, `delete_user`, `list_user_permissions` | `GET|PUT|DELETE /api/users[/{name}]`, `GET .../permissions` |
| Permissions | `set_permission`, `delete_permission` | `PUT|DELETE /api/permissions/{vhost}/{user}` |
| Policies | `list_policies` (+`_in_vhost`), `get_policy`, `set_policy`, `delete_policy`; same set for operator policies | `GET|PUT|DELETE /api/policies/...`, `/api/operator-policies/...` |
| Parameters | `list_parameters` (+`_in_vhost`), `get_parameter`, `set_parameter`, `delete_parameter` | `GET|PUT|DELETE /api/parameters/...` |
| Connections | `list_connections` (+`_paged`), `get_connection`, `close_connection`, `list_connection_channels` | `GET|DELETE /api/connections[/{name}]`, `GET .../channels` |
| Channels | `list_channels`, `get_channel` | `GET /api/channels[/{name}]` |
| Consumers | `list_consumers` (+`_in_vhost`) | `GET /api/consumers[/{vhost}]` |
| Definitions | `export_definitions` (+`_in_vhost`), `import_definitions` | `GET|POST /api/definitions[/{vhost}]` |
| Health | `health_check_alarms`, `health_check_local_alarms`, `health_check_port_listener`, `health_check_protocol_listener`, `health_check_node_is_quorum_critical`, `health_check_virtual_hosts` | `GET /api/health/checks/...` |
| Ops | `list_alarms`, `rebalance_queues` | `GET /api/alarms`, `POST /api/rebalance/queues` |

All path segments (vhosts, queue names, ...) are percent-encoded automatically — `DEFAULT_VHOST`
(`/`) becomes `%2F` on the wire.

## Examples

Two runnable examples live in `examples/`. Both default to
`http://localhost:15672` with `guest`/`guest` and honor the `RABBITMQ_URL`, `RABBITMQ_USER`, and
`RABBITMQ_PASS` environment variables:

```sh
# Cluster snapshot: whoami, overview, per-node resources, top-10 queues by depth.
cargo run --example monitor_queues

# Admin workflow: declare exchange + queue, bind, publish, verify, purge, clean up.
cargo run --example declare_and_bind
```

## Error handling

All fallible calls return `rabbitmqadmin_sdk::Result<T>` with a single `Error` enum:

- `Error::NotFound(ctx)` — the server returned 404; `ctx` identifies the missing resource
  (e.g. `queue 'q1' in vhost '/'`).
- `Error::Api { status, reason }` — any other non-2xx response; `reason` carries the server's
  response body. **Health checks report failure as 503**, so a failed check surfaces as
  `Error::Api { status: 503, .. }` with the failure JSON in `reason` — match on it rather than
  treating it as an unexpected error.
- `Error::Transport(..)` — HTTP-level failures (DNS, TLS, timeouts, connection refused).
- `Error::Deserialize { source, body }` — the response didn't match the expected schema; the raw
  body is retained for debugging.
- `Error::InvalidUrl(..)` — malformed base URL (must be `http://` or `https://`).

## Pagination

Large list endpoints come in two flavors: `list_x()` returns the full `Vec<T>`, while
`list_x_paged(&PaginationQuery)` returns a `Paginated<T>` envelope with `items`, `page`,
`page_size`, `total_count`, and `filtered_count`. `PaginationQuery` supports `page`, `page_size`,
`name` filtering, and `use_regex`. Note the server caps `page_size` at 500.

## License

[Apache-2.0](LICENSE)
