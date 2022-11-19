use std::sync::{Arc, RwLock};

use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
enum Action {
    Add,
    Del,
    List,
    Match,
}

#[derive(Debug, Clone, Serialize)]
struct Entry {
    kind: Action,
    user: String,
    security: String,
    side: Side,
    quantity: u64,
    price: u64,
}

#[derive(Debug, Clone, Serialize)]
enum Side {
    Buy,
    Sell,
}

type State = RwLock<Vec<Entry>>;

#[tokio::main]
async fn main() {
    let state: Arc<State> = Arc::new(State::default());

    let app = Router::new()
        .route("/", get(|| async { "To access the API got to /api" }))
        .route(
            "/api/:action",
            get({
                let state = Arc::clone(&state);
                move |action| api_get(action, state)
            })
            .post({
                let state = Arc::clone(&state);
                move |action| api_post(action, state)
            }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api_get(
    Path(action): Path<Action>,
    state: Arc<State>,
) -> Result<Json<Vec<Entry>>, StatusCode> {
    use Action::*;

    let read = state.read().expect("Valid Mutex");

    match action {
        List => Ok(Json(read.to_vec())),
        Match => Ok(Json(
            read.iter()
                .filter(|entry| entry.kind == Action::Match)
                .cloned()
                .collect(),
        )),
        Add | Del => Err(StatusCode::BAD_REQUEST),
    }
}

async fn api_post(
    Path(action): Path<Action>,
    state: Arc<State>,
) -> Result<&'static str, StatusCode> {
    use Action::*;

    let _write = state.write().expect("Valid Mutex");

    match action {
        Add => todo!(),
        Del => todo!(),
        List | Match => Err(StatusCode::BAD_REQUEST),
    }
}
