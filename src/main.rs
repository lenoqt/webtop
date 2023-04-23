use axum::{Server, Router, routing::get};



#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_get));
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    server.await.unwrap();
}

async fn root_get() -> &'static str {
    "Hello world"
}