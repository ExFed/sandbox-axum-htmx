use askama::Template;
use axum::{extract::Query, routing, serve, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", routing::get(index))
        .nest_service(
            "/favicon.ico",
            tower_http::services::ServeFile::new("assets/favicon/favicon.ico"),
        )
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct HelloParam {
    who: Option<String>,
}

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct HelloTemplate {
    pub who: String,
}

async fn index(Query(hello): Query<HelloParam>) -> HelloTemplate {
    let who = hello.who.unwrap_or("mom".into());
    HelloTemplate { who }
}
