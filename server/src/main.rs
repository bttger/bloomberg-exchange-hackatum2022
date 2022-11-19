use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnection, PgPoolOptions};
use std::{
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc},
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use uuid::Uuid;

// type State = RwLock<()>;
// type ServerResult<T> = Result<T, StatusCode>;

static ORDER_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Deserialize, Serialize)]
enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Deserialize, Serialize)]
struct Add {
    user: String,
    side: Side,
    stock: String,
    price: u64,
    quantity: u64,
}

#[derive(Deserialize, Serialize)]
struct Del {
    user: String,
    side: Side,
    stock: String,
    price: u64,
    quantity: u64,
}

#[derive(Deserialize, Serialize)]
struct ListFilter {
    user: Option<String>,
    side: Option<Side>,
    stock: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct MatchFilter {
    user: Option<String>,
    buyer: Option<String>,
    seller: Option<String>,
    stock: Option<String>,
}

#[tokio::main]
async fn main() {
    // let _state: Arc<State> = Arc::new(State::default());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Ok(database_pool) = PgPoolOptions::new().max_connections(4).connect("postgresql://hackatum2022@localhost:5432/hackatum2022").await else {
        tracing::error!("Failed to connect to test database");
        return;
    };
    let database_pool = Arc::new(database_pool);

    let app = Router::new()
        .route("/", get(|| async { "To access the API go to /api" }))
        .route("/api", get(api))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(Extension(database_pool));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api(
    ws: WebSocketUpgrade,
    Extension(pool): Extension<Arc<PgConnection>>,
    user_agent: Option<TypedHeader<axum::headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    ws.on_upgrade(|socket| handle_socket(socket, pool))
}

// TODO: Add update API route to easily change Pricing
#[derive(Serialize, Deserialize)]
enum WebSocketMessage {
    Add(Add),
    Del(Del),
    List(ListFilter),
    Match(MatchFilter),
}

async fn handle_socket(mut socket: WebSocket, _pool: Arc<PgConnection>) {
    while let Some(msg) = socket.recv().await {
        let Ok(msg) = msg else {tracing::warn!("Failed to recieve Message"); return;};
        // Parse Message
        let Ok(msg) = (match msg {
            Message::Text(msg) => serde_json::from_str::<WebSocketMessage>(&msg),
            Message::Binary(_) | Message::Ping(_) | Message::Pong(_) => {
                let _ = socket.close().await;
                tracing::warn!("Didn't get a text message, closing socket");
                return;
            },
            Message::Close(_) => return,
        }) else {
            let _ = socket.send(Message::Text("Invalid JSON".to_string())).await;
            let _ = socket.close().await;
            tracing::warn!("Got invalid JSON, closing socket");
            return;
        };

        match msg {
            WebSocketMessage::Add(Add {
                user,
                side,
                stock,
                price,
                quantity,
            }) => {
                sqlx::query!(
                    "INSERT INTO orders VALUES (?, ?, ?, ?, ?, ?);",
                    ORDER_ID,
                    chrono::offset::Utc::now().timestamp_micros(),
                    &user,
                    &stock,
                    quantity,
                    price
                )
            }
            WebSocketMessage::Del(_) => todo!("Del"),
            WebSocketMessage::List(_) => todo!("List"),
            WebSocketMessage::Match(_) => todo!("Match"),
        }
    }
}
