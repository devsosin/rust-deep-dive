use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

// Extractor Path, Query, Json, Form
// FromRequest trait implements - UrlEncoded &key=value / T : DeserializeOwned + 'static
// Runtime에서 느리지 않음 *monomorphization (단일형화, 컴파일 타임 프로세스로 - 고유한 인스턴스화에 대해 다형성 함수가 많은 단일형 함수로 대체)
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// PgConnection을 따로 설정하지 않아도 가져올 수 있는 이유?
// app state를 관리하기 위해 type-map 사용
// 고유한 유형 식별자에 대해 unique type identifer (TypeId::of로부터 얻은)
// 임의 데이터(Any type)를 저장하는 HashMap
// request 들어오면 명시해놓은(PgConnection) TypeId를 계산하고 type-map에 있는지 체크
// Any value로 가져온 뒤 type casting
// dependency injection과 동일?한 기술

// sqlx::query!의 Executor trait이 &mut PgConnection을 필요로 함. (&PgConnection이 아닌)
// PgConnection을 쓰면 mutable 참조로 가져와야하는데,
// mutable한 참조는 1개여야함. (unique) - 여러 자원이 수정하면 sync에 문제가 생기므로 (concurrency issue)
// 그럴려면 Mutex를 활용하여 해당 자원을 lock하는 식으로 sync를 맞춰야함.
// shared refrence to PgPool (pool of connections)
// pool에서 PgConnection을 가져오고, 가용하지 않으면 새로 생성하거나 free up 할 때까지 기다림
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
