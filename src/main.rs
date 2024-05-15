use {
    axum::{
        routing::{get_service, post},
        Router,
    },
    tokio::net::TcpListener,
    tower_http::services::ServeDir,
};

use std::{
    path::PathBuf,
    sync::mpsc::{channel, Sender},
    thread,
};

mod args;
mod camera_handler;
mod led_handler;
mod system_calls;

use crate::args::Arguments; // commandline arguments
use crate::camera_handler::camera_signals_handler;
use crate::led_handler::{drive_led, led_signals_handler, LedMode};

// #[derive(Debug)]

#[derive(Clone)]
pub struct AppState {
    pub sender: Sender<LedMode>,
}

#[tokio::main]
async fn main() {
    let args: Arguments = argh::from_env();
    let html_directory = PathBuf::from(args.html_folder);

    // maybe `sync_channel` (with back-pressure) instead?
    let (led_control_sender, led_control_receiver) = channel::<LedMode>();

    thread::spawn(move || drive_led(led_control_receiver));
    // spawn thread handling camera and making sure not too many images are taken

    let shared_state = AppState {
        sender: led_control_sender,
    };

    let app = Router::new()
        .route("/send_camera", post(camera_signals_handler))
        .route("/send_led", post(led_signals_handler))
        .fallback_service(routes_static(html_directory))
        .with_state(shared_state);

    let address = format!("0.0.0.0:{}", &args.portnumber);
    println!("starting TcpListener at {}", &address);
    let listener = TcpListener::bind(address).await.unwrap();

    // Start the server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn routes_static(path: PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(path.as_os_str())))
}
