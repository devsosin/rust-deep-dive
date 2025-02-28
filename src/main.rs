use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::config::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
