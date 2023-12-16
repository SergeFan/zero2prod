use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if failed to bind the address
    // Otherwise call .await on the server
    run(TcpListener::bind("127.0.0.1:6789").unwrap())?.await
}
