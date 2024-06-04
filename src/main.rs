/// this file is the `crate root`.
/// it declares which modules belong to this crate
/// and which elements (structs, functions, traits, constants)
/// are used within this file.
/// `use crate::*` are elements defined within this crate
/// everything else is a dependency from somewhere else
mod args;
mod buzzer;

use crate::{args::Arguments, buzzer::buzzer_signals_handler};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use {
    axum::{
        extract::{connect_info::ConnectInfo, State},
        response::IntoResponse,
        routing::{get, get_service, post},
        Json, Router,
    },
    axum_server::tls_rustls::RustlsConfig,
    serde::Serialize,
    tokio::sync::Mutex,
    tower_http::services::ServeDir,
};

/// The state which can later be shared between
/// route handler-functions.
#[derive(Clone)]
pub struct AppState {
    // maybe replace this by a semaphore
    pub buzzer_resource_lock: Arc<Mutex<()>>,
}

/// the main function is prepended with a macro which modifies
/// the whole function in order to turn it into an async (runtime?)'
/// I haven't dipped too deep into this one yet
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let args: Arguments = argh::from_env();
    let html_directory = PathBuf::from(args.html_folder);

    let shared_state = AppState {
        buzzer_resource_lock: Arc::new(Mutex::new(())),
    };

    let tls_config = RustlsConfig::from_pem_file(
        format!("./{}/cert.pem", args.tls_directory),
        format!("./{}/key.pem", args.tls_directory),
    )
    .await
    .expect(&format!(
        "error while loading tls certificate `cert.pem` and key `key.pem` in directory `{}`.",
        &args.tls_directory
    ));

    // setup router with routes and their respective handler-functions
    let app = Router::new()
        .route("/send_buzzer", post(buzzer_signals_handler))
        .route("/status", get(status_response))
        .fallback_service(routes_static(html_directory))
        .with_state(shared_state);

    let address = format!("0.0.0.0:{}", &args.portnumber);
    println!("Starting TcpListener at {}", &address);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Starting TcpListener at {}", addr);

    // Start the server
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Could not start server.");
}

fn routes_static(path: PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(path.as_os_str())))
}

/// the status-response which is serialized into json
/// by the #[derive(Serialize)]-macro
#[derive(Serialize)]
struct StatusResponse<'a> {
    buzzer_status: &'a str,
    server_status: &'a str,
}

/// the function which handles status responses
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
