use axum::Router;
use axum::routing::get;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}


pub fn day0_routes(router: Router) -> Router {
    router
        .route("/", get(hello_world))
        .route("/-1/seek", get(|| async {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::FOUND)
            .header("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")
            .body::<String>("".into())
            .unwrap()
    }))
}