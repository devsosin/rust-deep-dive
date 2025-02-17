use actix_web::{web, HttpResponse};

// Extractor Path, Query, Json, Form
// FromRequest trait implements - UrlEncoded &key=value / T : DeserializeOwned + 'static
// Runtime에서 느리지 않음 *monomorphization (단일형화, 컴파일 타임 프로세스로 - 고유한 인스턴스화에 대해 다형성 함수가 많은 단일형 함수로 대체)
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
