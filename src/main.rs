use axum::{
  routing::get,
  Router, response::Html
};



#[tokio::main]
async fn main() {

  let app = Router::new()
    .route("/api/hello", get(|| async { "Hello, World!" }))
    .route("/api/html", get(html_handler))
    ;

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn html_handler() -> Html<&'static str> {
  Html("<h1>It's from Axum!</h1>")
}