use std::fmt;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use cargo_manifest::Manifest;
use serde::Deserialize;
use serde_with::serde_as;
use shuttle_runtime::__internals::serde_json;

#[derive(Deserialize, Default)]
struct Metadata {
    #[serde(default)]
    pub orders: Vec<Order>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Order {
    pub item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    pub quantity: Option<u32>,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quantity {
            Some(num) => write!(f, "{}: {:?}", self.item, num),
            None => write!(f, ""),
        }
    }
}

async fn toml_orders(headers: HeaderMap, text: String) -> impl IntoResponse {
    let manifest = match headers.get("content-type") {
        Some(content_type) => {
            match content_type.to_str() {
                Ok("application/toml") => {
                    if let Ok(manifest) = toml::from_str::<Manifest<Metadata>>(&text) {
                        manifest
                    } else {
                        return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
                    }
                }
                Ok("application/json") => {
                    if let Ok(manifest) = serde_json::from_str::<Manifest<Metadata>>(&text) {
                        manifest
                    } else {
                        return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
                    }
                }
                Ok("application/yaml") => {
                    if let Ok(manifest) = serde_yaml::from_str::<Manifest<Metadata>>(&text) {
                        manifest
                    } else {
                        return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
                    }
                }
                _ => return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported content type".to_string())
            }
        }
        None => return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Content type not set".to_string()),
    };

    let package = manifest.package.unwrap();
    if !package
        .keywords
        .and_then(|k| k.as_local())
        .map(|k| k.contains(&"Christmas 2024".to_string()))
        .unwrap_or_default()
    {
        return (StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string());
    }

    let metadata = match package.metadata {
        Some(metadata) => metadata,
        None => return (StatusCode::NO_CONTENT, "No metadata found".to_string()),
    };

    let orders: String = metadata
        .orders
        .iter()
        .filter(|order| order.quantity.is_some())
        .map(|order| order.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    if orders.is_empty() {
        return (StatusCode::NO_CONTENT, "No orders found".to_string());
    }
    (StatusCode::OK, orders)
}

pub fn day5_routes(router: Router) -> Router {
    router.route("/5/manifest", post(toml_orders))
}