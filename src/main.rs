use axum::{extract::Query, response::Html, routing, serve, Router};
use rand::Rng;
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", routing::get(handle));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn handle(Query(range): Query<RangeParameters>) -> Html<String> {
    let num = rand::thread_rng().gen_range(range.start..range.end);
    Html(format!("{}", num))
}
