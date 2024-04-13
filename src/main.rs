use axum::{
    routing::get,
    http::StatusCode,
    Router,
    extract::Query,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct GreetingParams {
    name: String,
    fuck_boi: String
}

async fn hello_world(Query(params): Query<GreetingParams>) -> impl IntoResponse {
    let greeting = format!("Hello, {}, fart {}!", params.name, params.fuck_boi);
    (StatusCode::OK, greeting)
}

async fn holdens_fuckboi_function() -> impl IntoResponse {
    (StatusCode::OK, "Fart in my mouth please!")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world)).route("/anus", get(holdens_fuckboi_function));

    Ok(router.into())
}
