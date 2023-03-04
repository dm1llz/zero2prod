#[tokio::test]
async fn health_check_works() {
    let host = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{host}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("localhost:0").expect("Unable to bind to port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("localhost:{port}")
}
