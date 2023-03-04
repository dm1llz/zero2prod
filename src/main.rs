use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("localhost:8000").expect("Unable to bind to port");
    run(listener)?.await
}
