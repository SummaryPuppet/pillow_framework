# Pillow Framework

Is a web framework for rust

## Getting started

Add dependency

```toml
[dependencies]
pillow = "0.2.0"
tokio = {version = "1.23.0", features = ["full"]}
```

## Simple Server

```rust
use pillow::http::{MainRouter, Response};

#[tokio::main]
async fn main() {
  let mut router = MainRouter::new();

  router.get("/", |_request| Response::view("index"));

  let server = Server::new().unwrap();

  server.run(&router).await:
}
```

## Documentation

* [docs.rs](https://docs.rs/pillow/latest/pillow/)

## Lincese

MIT Lincese

## Contribution

