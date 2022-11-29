extern crate pillow;

use pillow::{database::connection::postgres::Postgres, routing::router::Router};

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| {
        let mut postgres = Postgres::new();

        postgres.create_table(
            "author",
            "(
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                country VARCHAR NOT NULL
        )",
        );

        let json = format!("{{}}");
        response.json(&json)
    });

    app.listen("5000").await;
}
