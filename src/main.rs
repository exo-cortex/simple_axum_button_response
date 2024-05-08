use std::path::PathBuf;
use tokio::net::TcpListener;
use {
    axum::{
        routing::{get_service, post},
        Router,
    },
    tower_http::services::ServeDir,
};

mod args;
mod system_calls;
use crate::args::Arguments;

async fn handle_post(word: String) {
    // Print the word received from the POST request
    match word.as_str() {
        "{\"button\":\"pressed\"}" => {
            println!("attempting to use camera.");
            let output = system_calls::update_image();
            println!("{}", output);
        }
        word => println!("Received: {}", word),
    }

    // Return a response, if needed
    // "Word received successfully".to_string()
}

#[tokio::main]
async fn main() {
    let args: Arguments = argh::from_env();

    let html_location = PathBuf::from(args.html_folder);

    // Create an Axum router with a route to handle POST requests to the "/send" endpoint
    let app = Router::new()
        .route("/send", post(handle_post))
        .fallback_service(routes_static(html_location));

    // Start the server

    let address = format!("0.0.0.0:{}", &args.portnumber);

    println!("starting TcpListener at {}", &address);
    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn routes_static(path: PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(path.as_os_str())))
}
