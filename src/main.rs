use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::BitXor;
use std::str::FromStr;
use axum::{routing::get, routing::post, Router};
use axum::extract::{Query};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use cargo_manifest::Manifest;
use serde::Deserialize;
use toml;

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

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[derive(Deserialize)]
struct FromKey {
    from: Ipv4Addr,
    key: Ipv4Addr,
}
#[derive(Deserialize)]
struct FromTo {
    from: Ipv4Addr,
    to: Ipv4Addr,
}
#[derive(Deserialize)]
struct Ipv6FromKey {
    from: Ipv6Addr,
    key: Ipv6Addr,
}
#[derive(Deserialize)]
struct Ipv6FromTo {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

async fn encryption(fromkey: Query<FromKey>) -> impl IntoResponse {
    let from = fromkey.from.octets();
    let key = fromkey.key.octets();

    let first = from.get(0).unwrap().wrapping_add(*key.get(0).unwrap());
    let second = from.get(1).unwrap().wrapping_add(*key.get(1).unwrap());
    let third = from.get(2).unwrap().wrapping_add(*key.get(2).unwrap());
    let fourth = from.get(3).unwrap().wrapping_add(*key.get(3).unwrap());

    Ipv4Addr::new(first, second, third, fourth).to_string()
}
async fn ipv6_encryption(fromkey: Query<Ipv6FromKey>) -> impl IntoResponse {
    let from = fromkey.from.segments();
    let key = fromkey.key.segments();

    let first = from.get(0).unwrap().bitxor(*key.get(0).unwrap());
    let second = from.get(1).unwrap().bitxor(*key.get(1).unwrap());
    let third = from.get(2).unwrap().bitxor(*key.get(2).unwrap());
    let fourth = from.get(3).unwrap().bitxor(*key.get(3).unwrap());
    let fifth = from.get(4).unwrap().bitxor(*key.get(4).unwrap());
    let sixth = from.get(5).unwrap().bitxor(*key.get(5).unwrap());
    let seventh = from.get(6).unwrap().bitxor(*key.get(6).unwrap());
    let eighth = from.get(7).unwrap().bitxor(*key.get(7).unwrap());

    Ipv6Addr::new(first, second, third, fourth, fifth, sixth, seventh, eighth).to_string()
}

async fn decryption(fromto: Query<FromTo>) -> impl IntoResponse {
    let from = fromto.from.octets();
    let to = fromto.to.octets();

    let first = to.get(0).unwrap().wrapping_sub(*from.get(0).unwrap());
    let second = to.get(1).unwrap().wrapping_sub(*from.get(1).unwrap());
    let third = to.get(2).unwrap().wrapping_sub(*from.get(2).unwrap());
    let fourth = to.get(3).unwrap().wrapping_sub(*from.get(3).unwrap());

    Ipv4Addr::new(first, second, third, fourth).to_string()
}

async fn ipv6_decryption(fromto: Query<Ipv6FromTo>) -> impl IntoResponse {
    let from = fromto.from.segments();
    let to = fromto.to.segments();

    let first = to.get(0).unwrap().bitxor(*from.get(0).unwrap());
    let second = to.get(1).unwrap().bitxor(*from.get(1).unwrap());
    let third = to.get(2).unwrap().bitxor(*from.get(2).unwrap());
    let fourth = to.get(3).unwrap().bitxor(*from.get(3).unwrap());
    let fifth = to.get(4).unwrap().bitxor(*from.get(4).unwrap());
    let sixth = to.get(5).unwrap().bitxor(*from.get(5).unwrap());
    let seventh = to.get(6).unwrap().bitxor(*from.get(6).unwrap());
    let eighth = to.get(7).unwrap().bitxor(*from.get(7).unwrap());

    Ipv6Addr::new(first, second, third, fourth, fifth, sixth, seventh, eighth).to_string()
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    let router = router.route("/-1/seek", get(|| async {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::FOUND)
            .header("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")
            .body::<String>("".into())
            .unwrap()
    }));
    let router = router.route("/2/dest", get(encryption));
    let router = router.route("/2/key", get(decryption));

    let router = router.route("/2/v6/dest", get(ipv6_encryption));
    let router = router.route("/2/v6/key", get(ipv6_decryption));

    let router = router.route("/5/manifest", post(toml_orders));
    Ok(router.into())
}
