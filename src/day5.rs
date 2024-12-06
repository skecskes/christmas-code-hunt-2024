use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use cargo_manifest::Manifest;
use serde::Deserialize;
use shuttle_runtime::__internals::serde_json;

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

    let Some(package) = (match headers.get("content-type") {
        Some(content_type) => {
            match content_type.to_str() {
                Ok("application/toml") => {
                    toml::from_str::<Manifest<Metadata>>(&text).ok()
                },
                Ok("application/json") => {
                    serde_json::from_str::<Manifest<Metadata>>(&text).ok()
                },
                Ok("application/yaml") => {
                    serde_yaml::from_str::<Manifest<Metadata>>(&text).ok()
                },
                _ => return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported content type".to_string())
            }
        },
        None => return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Content type not set".to_string()),
    }).and_then(|manifest| manifest.package) else {
        return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
    };

    if !package
        .keywords
        .and_then(|kws| kws.as_local())
        .map(|kws| kws.contains(&"Christmas 2024".to_string()))
        .unwrap_or_default() {
        return (StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string());
    }

    let Some(orders) = package.metadata.map(|m| {
        m.orders.into_iter()
            .filter(|order| order.quantity.is_some())
            .map(|order| {
                format!("{}: {}", order.item, order.quantity.unwrap())
            })
            .collect::<Vec<_>>()
        }) else {
            return (StatusCode::NO_CONTENT, "No orders found".to_string());
        };

    if orders.is_empty() {
        return (StatusCode::NO_CONTENT, "No orders found".to_string());
    }
    (StatusCode::OK, orders.join("\n"))
}

pub fn day5_routes(router: Router) -> Router {
    router.route("/5/manifest", post(toml_orders))
}