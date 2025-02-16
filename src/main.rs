use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// Responder trait implementor (strings, status codes, bytes, HttpResponse, ...)
// 해당 signatures를 꼭 따라야 하는 것은 아님 (특히 input에서)
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}


// Async Programming in Rust is build on top of Future trait
// All futures expose a poll method (최종적으로 결과반환을 위해 불려야하는 것)
// lazy 방식 (최대한 늦게 실행)

// this is macro (cargo expand), operates at the token level
// main purpose is code generation (compiler do this)
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Server - HttpServer -> server application backbone
    // 1. listening where? (TCP socket 127.0.0.1:8000)
    // 2. maximum number of concurrent connections
    // 3. enable TLS (transport layer security)?
    HttpServer::new( || {

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
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))

            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// cargo expand
// fn main() -> Result<(), std::io::Error> {
//     let body = async {
//         HttpServer::new(|| {
//                 App::new()
//                     .route("/", web::get().to(greet))
//                     .route("/{name}", web::get().to(greet))
//             })
//             .bind("127.0.0.1:8000")?
//             .run()
//             .await
//     };
//     #[allow(
//         clippy::expect_used,
//         clippy::diverging_sub_expression,
//         clippy::needless_return
//     )]
//     {
//         return tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .expect("Failed building the Runtime")
//             .block_on(body);
//     }
// }
