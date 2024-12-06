use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::BitXor;
use std::str::FromStr;

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

pub fn day2_routes(router: Router) -> Router {
    let router = router.route("/2/dest", get(encryption));
    let router = router.route("/2/key", get(decryption));

    let router = router.route("/2/v6/dest", get(ipv6_encryption));
    let router = router.route("/2/v6/key", get(ipv6_decryption));
    router
}