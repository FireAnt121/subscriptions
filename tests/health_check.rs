use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use ztp::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = ztp::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthCheck", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn returns_200_for_valid_data() {
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres");

    let client = reqwest::Client::new();

    let body = "name=fire%20ant&email=fireant%40gamil.com";

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute reqyest.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch savec subs");

    assert_eq!(saved.email, "fireant@gamil.com");
    assert_eq!(saved.name, "fire ant");
}

#[tokio::test]
async fn returns_400_for_data_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=fire%20ant", "missing the eamil"),
        ("email=fireant%40gamil.com", "missing the name"),
        ("", "missing the name and eamil"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute reqyest.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 bad request when the payload was {}",
            error_message
        );
    }
}
