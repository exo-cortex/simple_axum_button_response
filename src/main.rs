use tokio::net::TcpListener;
use {
    axum::{
        routing::{get_service, post},
        Router,
    },
    tower_http::services::ServeDir,
};

async fn handle_post(word: String) {
    // Print the word received from the POST request
    println!("Received word: {}", word);
    // Return a response, if needed
    // "Word received successfully".to_string()
}

#[tokio::main]
async fn main() {
    let mut state: u32 = 0;

    // Create an Axum router with a route to handle POST requests to the "/send" endpoint
    let app = Router::new()
        .route("/send", post(handle_post))
        .fallback_service(routes_static());

    // Start the server

    let listener = TcpListener::bind("127.0.0.1:4321").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./index.html")))
}
