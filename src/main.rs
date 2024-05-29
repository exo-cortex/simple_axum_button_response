use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use {
    axum::{
        extract::{connect_info::ConnectInfo, State},
        response::IntoResponse,
        routing::{get, get_service, post},
        Json, Router,
    },
    serde::Serialize,
    tokio::{net::TcpListener, sync::Mutex},
    tower_http::services::ServeDir,
};

mod args;
mod buzzer;

use crate::{args::Arguments, buzzer::buzzer_signals_handler};

#[derive(Clone)]
pub struct AppState {
    // maybe replace this by a semaphore
    pub buzzer_resource_lock: Arc<Mutex<()>>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let args: Arguments = argh::from_env();
    let html_directory = PathBuf::from(args.html_folder);

    let shared_state = AppState {
        buzzer_resource_lock: Arc::new(Mutex::new(())),
    };

    // setup router
    let app = Router::new()
        .route("/send_buzzer", post(buzzer_signals_handler))
        .route("/status", get(status_response))
        .fallback_service(routes_static(html_directory))
        .with_state(shared_state);

    let address = format!("0.0.0.0:{}", &args.portnumber);
    println!("Starting TcpListener at {}", &address);
    let listener = TcpListener::bind(address).await.unwrap();

    // Start the server
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Could not start server.");
}

fn routes_static(path: PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(path.as_os_str())))
}

#[derive(Serialize)]
struct StatusResponse<'a> {
    buzzer_status: &'a str,
    server_status: &'a str,
}

async fn status_response(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let cloned_buzzer_in_use = state.buzzer_resource_lock.clone();

    let busy = match cloned_buzzer_in_use.try_lock() {
        Ok(_) => false,
        Err(_) => true,
    };

    println!("\'get\' request to \'/status\' from {}", addr);

    Json(StatusResponse {
        buzzer_status: if busy { "buzzer_busy" } else { "buzzer_free" },
        server_status: "online",
    })
}
