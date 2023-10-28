use axum::{http, extract};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: now,
            updated_at: now
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>
    axum::Json(payload): axum::Json<CreateQuote>,
) -> http::StatusCode {
    println!("{:?}", payload);
    http::StatusCode::CREATED
}
