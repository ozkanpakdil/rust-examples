# rust-examples

different rust example projects

## REST API framework examples

The same minimal example — a single `GET /hello` endpoint returning `{"name": "rust-<framework>", "year": <current year>}` on port 8080 — implemented with each framework, based on the [10 Best Rust Web Frameworks](https://snapbyte.dev/blog/rust-web-frameworks) list:

| Folder | Framework |
|--------|-----------|
| `actix-rest-api` | [Actix Web](https://actix.rs) — high-throughput async framework with rich middleware support |
| `axum-rest-api` | [Axum](https://github.com/tokio-rs/axum) — Tokio's ergonomic, Tower-based web framework |
| `rocket-rest-api` | [Rocket](https://rocket.rs) — ergonomics-first framework with type-safe routing |
| `warp-rest-api` | [Warp](https://github.com/seanmonstar/warp) — composable filter-based web framework |
| `salvo-rest-api` | [Salvo](https://salvo.rs) — batteries-included framework with HTTP/2 & HTTP/3 support |
| `poem-rest-api` | [Poem](https://poem.rs) — full-featured framework with strong OpenAPI ecosystem |
| `gotham-rest-api` | [Gotham](https://gotham.rs) — stability- and safety-focused async framework |
| `viz-rest-api` | [Viz](https://viz.rs) — fast, lightweight web framework |
| `trillium-rest-api` | [Trillium](https://trillium.rs) — modular async web toolkit built around the `Conn` handler model |

## Other examples

| Folder | What it does |
|--------|--------------|
| `arti_whois` | Whois lookup web service (warp) that routes requests through the Tor network via `arti-client` |
| `download_upload` | Streaming file download with `reqwest` and a terminal progress bar (`indicatif`) |
| `hackerrank` | Solutions to [HackerRank](https://www.hackerrank.com) challenges (security function inverses) |
| `juniper` | GraphQL server built with `juniper` on top of actix-web |
| `jwt-server` | JWT auth server with axum — bcrypt password hashing, token issuance and validation |
| `piston_render_text` | Text rendering demo using the [Piston](https://www.piston.rs) 2D game engine |
| `postgres_list_sessions` | Terminal dashboard (ratatui) listing active PostgreSQL sessions |
| `rucene_test` | Full-text search experiment with [rucene](https://crates.io/crates/rucene), a Rust port of Apache Lucene |
| `tantivy_test` | Full-text search experiment with [tantivy](https://crates.io/crates/tantivy) |
| `websocket_echo_server` | Minimal WebSocket echo server with `tokio-tungstenite` |