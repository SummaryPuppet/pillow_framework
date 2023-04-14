# Pillow Framework

Is a web framework for rust

## Getting started

Add dependency

```toml
[dependencies]
pillow = "0.2.0"
tokio = {version = "1.23.0", features = ["full"]}
```

### With Tokio

```rust
use pillow::http::Router;

#[tokio::main]
async fn main() {
  let mut app = Router::new();

  app.get("/", |request, response| response.view("index"));

  app.listen("5000").await;
}
```

### With Async_std

```rust
use pillow::http::Router;

#[async_std::main]
async fn main() {
  let mut app = Router::new();

  app.get("/", |request, response| response.view("index"));

  app.listen("5000").await;
}
```

## Documentation

* [docs.rs](https://docs.rs/pillow/latest/pillow/)

## Lincese

MIT Lincese

## Contribution

