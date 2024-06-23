use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use ztp::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to get configuration  file");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    ztp::startup::run(listener, connection)?.await
}
