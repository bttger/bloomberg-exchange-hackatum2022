use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
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
        .route("/api", get(api));
    // TODO: Add update API route to easily change Pricing

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

#[derive(Serialize, Deserialize)]
enum WebSocketMessage {
    Add(Add),
    Del(Del),
    List(ListFilter),
    Match(MatchFilter),
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let Ok(msg) = msg else {return;};

        // Parse Message

        // Reply
        if socket.send(msg).await.is_err() {
            // Client Disconnected
            return;
        }
    }
}
