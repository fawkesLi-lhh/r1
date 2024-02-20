use axum::{routing::get, Router};
use std::net::SocketAddr;
#![feature(coroutines)]
fn generator() -> impl Iterator<Item = i32> {
    let mut i = 0;
    loop {
        i += 1;
        yield i;
    }
}

#[tokio::main]
async fn main() {
    for num in generator().take(5) {
                println!("{}", num);
    }
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
            .unwrap();
}

async fn handler() -> String {
    "body".to_string()
}
