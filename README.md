# Pillow Framework

Is a web framework for rust

## Getting started

Add dependencie

```bash
cargo add pillow
```

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
