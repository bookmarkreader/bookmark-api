use graphul::{Graphul, http::Methods};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async {
        "Hello, Bookmark Web API 👋!"
    });

    app.run("127.0.0.1:8000").await;
}