# Pillow Framework

Is a web framework for rust

## Getting started

Add dependency

```toml
[dependencies]
pillow = "0.3.0"
tokio = {version = "1.23.0", features = ["full"]}
```

## Simple Server

```rust
use pillow::http::*;

#[controller(method = "GET", path = "/")]
fn index(){
  Response::text("hello")
}

#[tokio::main]
async fn main() {
  let mut router = MainRouter::new();

  router.add_route(route!(index {}));
  router.get("/users", |_request| Response::text("users"));

  let server = Server::default();

  server.run(&router).await:
}
```

## Documentation

* [docs.rs](https://docs.rs/pillow/latest/pillow/)

## Lincese

MIT Lincese

## Contribution