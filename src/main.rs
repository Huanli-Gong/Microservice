use axum::extract::Path;
use axum::{routing::get, Json, Router};
use serde_json::json;
use rust_axum_integer_roman_microservice::int_to_roman;

//Root Route for Roman Numeral Converter
async fn root() -> &'static str {
    "
    Roman Numeral Converter

    **Primary Route:**
    /convert/integer
    "
}

async fn convert(Path(number): Path<u32>) -> impl axum::response::IntoResponse {
    let roman_numeral = int_to_roman(number);
    let json = json!({
        "integer": number,
        "roman_numeral": roman_numeral
    });
    Json(json)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/convert/:number", get(convert));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
