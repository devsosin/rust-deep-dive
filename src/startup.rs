use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listner: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // web::Data wraps in an Atomic Reference Counted pointer
    // Arc는 원본을 복사해서 전달하는 것이 아닌 pointer를 전달
    // active 참조 카운트를 1 증가시키고 감싸고있는 값의 메모리 주소 복사를 전달
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connection);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get pointer copy
            .app_data(connection.clone())
    })
    .listen(listner)?
    .run();

    Ok(server)
}
