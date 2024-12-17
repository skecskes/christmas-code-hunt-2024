use std::time::{SystemTime, UNIX_EPOCH};
use axum::response::IntoResponse;
use axum::Router;
use axum::extract::Json;
use axum::http::{StatusCode};
use axum::routing::{get, post};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const SECRET_KEY: &[u8] = b"secret";

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    data: Value,
}

pub async fn wrap(
    Json(body): Json<Value>
   ) -> impl IntoResponse {
    // Set the expiration time as 1 hour from now
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 3600;

    let claims = Claims {
        exp: expiration,
        data: body,
    };

    // creates a valid JSON Web token (JWT) that contains the input JSON
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(SECRET_KEY.as_ref())
    ).unwrap();

    // return 200 OK with the token in set-cookie header called gift
    axum::http::Response::builder()
        .header("Set-Cookie", format!("gift={}", token))
        .body("OK".to_string())
        .unwrap()
}

pub async fn unwrap(
    jar: CookieJar,
) -> impl IntoResponse {
    // get the token from key `gift` from the cookie
    if let Some(token) = jar.get("gift") {
        // decode the token
        let token_data = jsonwebtoken::decode::<Claims>(
            &token.value(),
            &jsonwebtoken::DecodingKey::from_secret(SECRET_KEY.as_ref()),
            &jsonwebtoken::Validation::default()
        );

        if let Ok(decoded) = token_data {
                axum::http::Response::builder()
                    .body(serde_json::to_string(&decoded.claims.data).unwrap())
                    .unwrap()
            } else {
                // print the error message
                println!("{:?}", token_data.err());
                axum::http::Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("Invalid token".to_string())
                    .unwrap()
            }
    } else {
        axum::http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("No gift cookie found".to_string())
            .unwrap()
    }
}



pub fn day16_routes() -> Router {

    Router::new()
        .route("/16/wrap", post(wrap))
        .route("/16/unwrap", get(unwrap))
}