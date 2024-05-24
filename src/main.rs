use {
    axum::{
        extract::State,
        response::IntoResponse,
        routing::{get, get_service, post},
        Router,
    },
    tokio::net::TcpListener,
    tower_http::services::ServeDir,
};

use std::{
    path::PathBuf,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

// define modules (here: files) that belong to this crate
mod args;
mod buzzer;
mod camera_handler;
mod led;
mod use_libcamera;

use crate::{
    args::Arguments,
    buzzer::buzzer_signals_handler,
    camera_handler::camera_signals_handler,
    led::{
        led_signals_handler,
        on_off_led::{drive_led, LedMode},
        pwm_led::{drive_pwm_led, PwmLedMode},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub sender: Sender<LedMode>,
    pub led_current_state: Arc<Mutex<LedMode>>,
    pub pwm_sender: Sender<PwmLedMode>,
    pub pwm_current_state: Arc<Mutex<PwmLedMode>>,
    pub camera_in_use: Arc<Mutex<bool>>,
    pub buzzer_in_use: Arc<Mutex<bool>>,
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
#[tokio::main]
async fn main() {
    let args: Arguments = argh::from_env();
    let html_directory = PathBuf::from(args.html_folder);

    let (led_control_sender, led_control_receiver) = channel::<LedMode>();
    let (pwm_control_sender, pwm_control_receiver) = channel::<PwmLedMode>();

    let led_current_state = Arc::new(Mutex::new(LedMode::Off));
    let pwm_current_state = Arc::new(Mutex::new(PwmLedMode::Off));

    let shared_state = AppState {
        sender: led_control_sender,
        led_current_state: led_current_state.clone(),
        pwm_sender: pwm_control_sender,
        pwm_current_state: pwm_current_state.clone(),
        camera_in_use: Arc::new(Mutex::new(false)),
        buzzer_in_use: Arc::new(Mutex::new(false)),
    };

    thread::spawn(move || drive_led(led_control_receiver));
    thread::spawn(move || drive_pwm_led(pwm_current_state.clone(), pwm_control_receiver));

    // setup router
    let app = Router::new()
        .route("/send_camera", post(camera_signals_handler))
        .route("/send_led", post(led_signals_handler))
        .route("/send_buzzer", post(buzzer_signals_handler))
        .route("/status", get(status_response))
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

async fn status_response(State(state): State<AppState>) -> impl IntoResponse {
    let pwm_current_state = state.pwm_current_state.clone();
    let pwm_state = pwm_current_state.lock().unwrap();

    let response = match *pwm_state {
        PwmLedMode::Off => String::from("off"),
        PwmLedMode::On => String::from("on"),
        PwmLedMode::Half => String::from("half"),
        PwmLedMode::BlinkEqualOnOff { period_ms } => {
            format!("blink_with_period_{}ms", period_ms)
        }
        PwmLedMode::Breath { period_ms } => {
            format!("(testing) breath {}ms", period_ms)
        }
        PwmLedMode::BreathLinear { period_ms } => {
            format!("(testing) breath {}ms", period_ms)
        }
        PwmLedMode::RisingSawTooth { period_ms } => {
            format!("(testing) sawtooth {}ms", period_ms)
        }
        PwmLedMode::RisingSawToothLinear { period_ms } => {
            format!("(testing) sawtooth (linear) {}ms", period_ms)
        }
        // todo
        _ => String::from(""),
    };

    drop(pwm_state);
    response
}
