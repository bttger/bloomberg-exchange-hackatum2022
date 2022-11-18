use axum::{extract::Path, routing::get, Router};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum Action {
    Add,
    Del,
    List,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/api/:action", get(api_get).post(api_post));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api_get(Path(action): Path<Action>) {
    use Action::*;
    match action {
        List => todo!("List handler"),
        Add | Del => todo!("Err message"),
    }
}

async fn api_post(Path(action): Path<Action>) {
    use Action::*;
    match action {
        Add | Del => todo!("Handler"),
        List => todo!("Err message"),
    }
}
