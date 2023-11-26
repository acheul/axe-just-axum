use axum::{
  routing::get,
  Router, response::Html
};



#[tokio::main]
async fn main() {

  let app = Router::new()
    .route("/api/hello", get(|| async { "Hello, World!" }))
    .route("/", get(a_handler))
    ;

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn a_handler() -> Html<&'static str> {

  let s = r#"
    <main style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; width: 100%; margin: 0;">
      <div style="display: flex; flex-direction: column; align-items: center; justify-content: flex-start;">
        <a href="https://github.com/tokio-rs/axum">
          <img src="https://avatars.githubusercontent.com/u/20248544" alt="https://github.com/tokio-rs/axum" size="100" height="100">
        </a>
        <p style="font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; user-select: none; font-size: 1.4rem; font-weight: bold;">just an axum</p>
      </div>
    </main>
  "#;

  Html(s)
}