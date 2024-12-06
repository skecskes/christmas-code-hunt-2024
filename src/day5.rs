use std::str::FromStr;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use cargo_manifest::Manifest;
use serde::Deserialize;

#[derive(serde::Deserialize)]
struct Toml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(default)]
    metadata: Metadata,
}

#[derive(Deserialize, Default)]
struct Metadata {
    orders: Vec<Order>,
}

#[serde_with::serde_as]
#[derive(Deserialize)]
struct Order {
    item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    quantity: Option<usize>,
}

async fn toml_orders(headers: HeaderMap, text: String) -> impl IntoResponse {
    if headers.get("content-type") != Some(&"application/toml".parse().unwrap()) {
        return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Invalid content type".to_string());
    }

    match toml::from_str::<Toml>(&text) {
        Ok(toml) => {
            if !toml.package.keywords.contains(&"Christmas 2024".to_string()) {
                return (StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string());
            }
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, e.to_string());
        }
    }

    if Manifest::from_str(&text).is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
    }


    match toml::from_str::<Toml>(&text) {
        Ok(toml) => {
            let result: String = toml.package.metadata.orders
                .into_iter()
                .filter_map(|order| {
                    if let Some(quantity) = order.quantity {
                       Some(format!("{}: {}", order.item, quantity))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            if result.is_empty() {
                 return (StatusCode::NO_CONTENT, "No orders found".to_string());
            }
            (StatusCode::OK, result)
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, e.to_string())
        }
    }
}

pub fn day5_routes(router: Router) -> Router {
    router.route("/5/manifest", post(toml_orders))
}