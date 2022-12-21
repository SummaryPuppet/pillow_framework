# Pillow Framework

Is a web framework for rust

## Getting started

Add dependency

```bash
cargo add pillow
```

or

```toml
[dependencies]
pillow = "0.2.0"
tokio = {version = "1.23.0", features = ["full"]}
```

### With Tokio

```rust
use pillow::http::router::Router;

#[tokio::main]
fn main() {
let app = Router::new();

app.get("/", |request, response| response.view("index"));

app.listen("5000").await;
}
```

### With Async_std

```rust
use pillow::http::router::Router;

#[async_std::main]
fn main() {
let app = Router::new();

app.get("/", |request, response| response.view("index"));

app.listen("5000").await;
}
```

## Documentation

* [docs.rs](https://docs.rs/pillow/latest/pillow/)

## Lincese

MIT Lincese

## Contribution

### For developers

clone project

```bash
git clone github.com/SummaryPuppet/pillow_framework.git
cd pillow_framework
```

and execute

```bash
cargo run
```
