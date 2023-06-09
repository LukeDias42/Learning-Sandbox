use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind to port 7878");
    run(listener)?.await
}
