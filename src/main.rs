use error::Result;
use salvo::prelude::*;

mod error;
mod router;

#[handler]
async fn hello_world() -> Result<&'static str> {
    Ok("hello world")
}

#[tokio::main]
async fn main() {
    let router = Router::new().push(router::user::router());
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}
