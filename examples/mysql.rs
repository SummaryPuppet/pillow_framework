use pillow::{database::connection::mysql::Mysql, routing::router::Router};

extern crate pillow;

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, response| {
        let mut mysql = Mysql::new();

        mysql.create_table(
            "author",
            "(
                id PRIMARY KEY 
                name VARCHAR(200) NOT NULL
                country VARCHAR(200) NOT NULL
        )",
        );

        response.text("Table created")
    });

    app.listen("5000").await
}
