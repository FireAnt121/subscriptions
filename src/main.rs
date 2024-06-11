use std::net::TcpListener;

use ztp::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to get configuration  file");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    ztp::startup::run(listener)
        .expect("Failed to bind address")
        .await
}
