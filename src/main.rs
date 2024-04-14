use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
#[derive(Clone)]
pub struct AppState {
    db: PgPool
}

impl AppState {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[derive(Deserialize)]
struct GreetingParams {
    name: String,
    fuck_boi: String,
}

async fn hello_world(Query(params): Query<GreetingParams>) -> impl IntoResponse {
    let greeting = format!("Hello, {}, fart {}!", params.name, params.fuck_boi);
    (StatusCode::OK, greeting)
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] conn_string: String) -> shuttle_axum::ShuttleAxum {
    let db = PgPoolOptions::new().connect(&conn_string).await.expect("Fucked up!!!");

    let state = AppState::new(db);

    let router = Router::new()
        .route("/hello", get(hello_world))
        .with_state(state);

    Ok(router.into())
}
