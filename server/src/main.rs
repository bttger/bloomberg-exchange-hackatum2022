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
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize, Serialize)]
enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Side {
    fn to_i16(&self) -> i16 {
        match self {
            Self::Buy => 0,
            Self::Sell => 1,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Add {
    user: String,
    side: Side,
    stock: String,
    price: i64,
    quantity: i64,
}

#[derive(Deserialize, Serialize)]
struct Del {
    user: String,
    side: Side,
    stock: String,
    price: i64,
    quantity: i64,
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

type Database = tokio_postgres::Client;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=hackatum2022", tokio_postgres::NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(err) = connection.await {
            tracing::error!("Failed to establish database connection: {err}")
        }
    });

    let database_pool = Arc::new(client);

    let app = Router::new()
        .route("/", get(api))
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
    Extension(pool): Extension<Arc<Database>>,
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

async fn handle_socket(mut socket: WebSocket, pool: Arc<Database>) {
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
                if let Err(err) = pool.query(
                            "INSERT INTO orders (time_, user_id, type_, exec_type, symbol, amount, price) VALUES ($1, $2::VARCHAR, $3, $4::SMALLINT, $5::VARCHAR, $6::BIGINT, $7::BIGINT);",
                            &[&chrono::offset::Utc::now().timestamp(),
                            &user,
                            &side.to_i16(),
                            &0i16,
                            &stock,
                            &quantity,
                            &price]
                        ).await {
                    tracing::warn!("Failed to execute query: {err}");
                }
                // let query = sqlx::query("INSERT INTO orders (time_, user_id, type_, exec_type, symbol, amount, price) VALUES (?, ?, ?, ?, ?, ?, ?);")
                //         .bind(chrono::offset::Utc::now().timestamp_micros())
                //         .bind(&user)
                //         .bind(match side { Side::Buy => 0, Side::Sell => 1})
                //         .bind(0)
                //         .bind(&stock)
                //         .bind(quantity)
                //         .bind(price)
                //         ;
                // tracing::debug!("Running query: {}", query.sql());
                // query.fetch_all(pool.deref()).await.expect("valid query");
            }
            WebSocketMessage::Del(_) => todo!("Del"),
            WebSocketMessage::List(_) => todo!("List"),
            WebSocketMessage::Match(_) => todo!("Match"),
        }
    }
}
