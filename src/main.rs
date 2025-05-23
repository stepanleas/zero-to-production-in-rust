use sqlx::{PgPool};
use std::net::TcpListener;
use env_logger::Env;
use zero_to_production_in_rust::configuration::get_configuration;
use zero_to_production_in_rust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, connection_pool)?.await
}