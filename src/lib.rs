use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// impl Responder에서 HttpResponse로 변경
// 성능상 차이는 없고, 스타일적 선택임
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Extractor Path, Query, Json, Form
// FromRequest trait implements - UrlEncoded &key=value / T : DeserializeOwned + 'static
// Runtime에서 느리지 않음 *monomorphization (단일형화, 컴파일 타임 프로세스로 - 고유한 인스턴스화에 대해 다형성 함수가 많은 단일형 함수로 대체)
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listner)?
    .run();

    Ok(server)
}
