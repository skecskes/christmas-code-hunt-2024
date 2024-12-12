mod day0;
mod day2;
mod day5;
mod day9;

use axum::{Router};
use crate::day0::day0_routes;
use crate::day2::day2_routes;
use crate::day5::day5_routes;
use crate::day9::day9_routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new();
    let router = day0_routes(router);
    let router = day2_routes(router);
    let router = day5_routes(router);
    let router = router.merge(day9_routes());
    Ok(router.into())
}
