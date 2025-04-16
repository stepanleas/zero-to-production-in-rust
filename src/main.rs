use std::net::TcpListener;
use zero_to_production_in_rust::configuration::get_configuration;
use zero_to_production_in_rust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}