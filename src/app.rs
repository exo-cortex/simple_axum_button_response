use crate::{
    buzzer::buzzer_signals_handler,
    camera::camera_signals_handler,
    led::{led_signals_handler, pwm_led::PwmLedMode},
    matrix_display::matrix_display_signals_handler,
    matrix_display::MatrixDisplayMode,
    status_response::status_response,
    website::routes_static,
};

use {
    axum::{
        routing::{get, post},
        Router,
    },
    std::{
        path::PathBuf,
        sync::{mpsc::Sender, Arc},
    },
    tokio::sync::Mutex,
};

#[derive(Clone)]
pub struct AppState {
    pub pwm_led_sender: Sender<PwmLedMode>,
    pub pwm_current_state: Arc<Mutex<PwmLedMode>>,
    pub buzzer_resource_lock: Arc<Mutex<()>>,
    pub camera_latest_image: Arc<Mutex<String>>,
    pub matrix_sender: Sender<MatrixDisplayMode>,
}

pub fn initialize_app(shared_state: AppState, html_directory_path: &PathBuf) -> Router {
    Router::new()
        .route("/send_buzzer", post(buzzer_signals_handler))
        .route("/send_camera", post(camera_signals_handler))
        .route("/send_led", post(led_signals_handler))
        .route("/send_matrix_display", post(matrix_display_signals_handler))
        .route("/status", get(status_response))
        .fallback_service(routes_static(&html_directory_path)) // serve static website
        .with_state(shared_state)
}
