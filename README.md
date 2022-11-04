# Pillow Framework

Is a minimalist framework for rust inspired in Expressjs

## Getting started

Add dependencie

```bash
cargo add pillow
```

```rust
let app = Router::new();

app.get("/", |request, response| response.view("index.html"));

app.listen("5000");
```

## Lincese

MIT Lincese

## Contribution

### For developers

clone project

```bash
git clone github.com/SummaryPuppet/pillow_framework.git
cd sunny_framework
```

and execute

```bash
cargo run
```
