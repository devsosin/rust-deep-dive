use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

// Responder trait implementor (strings, status codes, bytes, HttpResponse, ...)
// 해당 signatures를 꼭 따라야 하는 것은 아님 (특히 input에서)
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    // Server - HttpServer -> server application backbone
    // 1. listening where? (TCP socket 127.0.0.1:8000)
    // 2. maximum number of concurrent connections
    // 3. enable TLS (transport layer security)?
    let server = HttpServer::new( || {

        // Application - App
        // logics (routing, middlewares, request handlers, etc.)
        // take request (input) spit to response
        // builder pattern
        App::new()
            // Endpoint - Route
            // path, a string, can use /{name} for dynamic path segments
            // route an instance of the Route struct

            // Route combines a handler with a set of guards
            // Guards. request match conditions passed over handler (trait : Guard::check)
            // `web::get()` <- short-cut for `Route::new().guard(Guard::Get())`

            // 새 요청이 들어오면 route에서 돌면서 `path`와 `guard` 모두를 만족시키는 하나를 찾음
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))

            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
