use axum::Json;
use serde::Deserialize;

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

#[tokio::main]
async fn main() {
    let args: Arguments = argh::from_env();

    let html_location = PathBuf::from(args.html_folder);

    // Create an Axum router with a route to handle POST requests to the "/send" endpoint
    let app = Router::new()
        .route("/send", post(handler_camera_button))
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

#[derive(Debug, Deserialize)]
struct ButtonParams {
    signal: Option<String>,
}

async fn handler_camera_button(Json(params): Json<ButtonParams>) {
    // println!("received: {:?}", params);

    match params.signal.as_deref() {
        Some("camera_button_pressed") => {
            println!("camera button press detected, attempting to use camera...");
            let output = system_calls::update_image();
            println!("done. status: {}", output);
        }
        Some("clicked") => {
            println!("button clicked, not yet assigned to server function");
            let output = system_calls::update_image();
            println!("done {}", output);
        }
        Some(something) => println!("received unexpected signal: {}", something),
        None => println!("camera_button signal received but contained nothing."),
    }
}
