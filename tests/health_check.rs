use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let adress = spawn_app();

    // We need to bring in "reqwest"
    // to perform HTTP requests agains our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", adress))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port: u16 = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding list
    let _ = actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
