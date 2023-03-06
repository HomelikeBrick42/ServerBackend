use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/seconds_since_start", post(seconds_since_start_get))
        .fallback(fallback)
        .with_state(AppState {
            start_time: std::time::Instant::now(),
        });

    let server = Server::bind(&"127.0.0.1:1234".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    start_time: std::time::Instant,
}

async fn root_get() -> Response {
    const ROOT_PATH: &str = "src/index.html";
    // TODO: maybe cache this when not in debug builds?
    match tokio::fs::read_to_string(ROOT_PATH).await {
        Ok(content) => Html(content).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: Unable to find file '{ROOT_PATH}': {error}"),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct SecondsSinceStartPostData {
    random_number: i32,
}

#[derive(Serialize)]
struct SecondsSinceStartData {
    seconds: f64,
}

async fn seconds_since_start_get(
    State(state): State<AppState>,
    Json(post_data): Json<SecondsSinceStartPostData>,
) -> Json<SecondsSinceStartData> {
    _ = post_data.random_number;
    // println!("Post data random number: {}", post_data.random_number);
    let time = state.start_time.elapsed().as_secs_f64();
    Json(SecondsSinceStartData { seconds: time })
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("Unable to find route '{}'", uri),
    )
}
