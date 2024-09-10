mod notes;
mod routes;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    const SERVER_ADDR: &str = "127.0.0.1:8080";
    println!("Hola Caracola!");

    let listener = TcpListener::bind(SERVER_ADDR)
        .await
        .expect("Unable to create listener");
    axum::serve(listener, routes::app()).await.unwrap();
}
