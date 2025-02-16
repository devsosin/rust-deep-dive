use std::net::TcpListener;

use zero2prod::run;

// Async Programming in Rust is build on top of Future trait
// All futures expose a poll method (최종적으로 결과반환을 위해 불려야하는 것)
// lazy 방식 (최대한 늦게 실행)

// this is macro (cargo expand), operates at the token level
// main purpose is code generation (compiler do this)
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("http://127.0.0.1:8000")
        .expect("Failed to bind port 8000");
    run(listener)?.await
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
