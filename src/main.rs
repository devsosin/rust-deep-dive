use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::config::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.connect_options());

    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
