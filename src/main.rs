use salvo::prelude::*;

mod db;
mod error;
mod router;


#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env info load failed.");

    db::init_db_pool().await;

    let root = Router::new().push(router::user::router());

    // start server
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(root)
        .await;
}
