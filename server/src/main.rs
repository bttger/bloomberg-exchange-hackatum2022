use std::sync::{Arc, RwLock};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type State = RwLock<()>;
type ServerResult<T> = Result<T, StatusCode>;

#[derive(Deserialize, Serialize)]
enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Deserialize, Serialize)]
struct Add {
    user: Uuid,
    side: Side,
    stock: String,
    price: u64,
    quantity: u64,
}

#[derive(Deserialize, Serialize)]
struct Del {}

#[derive(Deserialize, Serialize)]
struct ListFilter {
    user: Option<Uuid>,
    side: Option<Side>,
    stock: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct MatchFilter {}

#[tokio::main]
async fn main() {
    let state: Arc<State> = Arc::new(State::default());

    let app = Router::new()
        .route("/", get(|| async { "To access the API got to /api" }))
        .route(
            "/api/match",
            get({
                let state = Arc::clone(&state);
                move |body| get_match(body, state)
            }),
        )
        .route(
            "/api/list",
            get({
                let state = Arc::clone(&state);
                move |body| get_list(body, state)
            }),
        )
        .route(
            "/api/del",
            post({
                let state = Arc::clone(&state);
                move |body| post_del(body, state)
            }),
        )
        .route(
            "/api/add",
            post({
                let state = Arc::clone(&state);
                move |body| post_add(body, state)
            }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post_add(Json(_body): Json<Add>, _state: Arc<State>) -> ServerResult<()> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn post_del(Json(_body): Json<Del>, _state: Arc<State>) -> ServerResult<()> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn get_list(Json(_body): Json<ListFilter>, _state: Arc<State>) -> ServerResult<()> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn get_match(Json(_body): Json<MatchFilter>, _state: Arc<State>) -> ServerResult<()> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
