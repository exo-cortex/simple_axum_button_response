// `main.rs`
// this file is the "crate root".
// it declares which modules belong to this crate
// and which elements (structs, functions, traits, constants)
// are used within this file.
mod app; // the main app
mod args; // commandline-args
mod buzzer; // operate the buzzer
mod camera; // operate the camera
mod led; // operate the led via pwm (very inefficient right now)
mod matrix_display; // operate the matrix_display
mod status_response; // inform clients about status
mod use_libcamera;
mod website; // serve the static website

// `use crate::*` brings elements into scope
use crate::{
    app::AppState,
    args::Arguments,
    led::pwm_led::{drive_pwm_led, PwmLedMode},
    matrix_display::drive_matrix_display,
    matrix_display::MatrixDisplayMode,
};

// bring into scope elements from inside dependencies
use {
    axum::Router,
    axum_server::tls_rustls::RustlsConfig,
    std::{
        net::SocketAddr,
        path::PathBuf,
        sync::{mpsc::channel, Arc},
        thread,
    },
    tokio::sync::Mutex,
};

/// The state which can later be shared between
/// route handler-functions.

/// the main function is prepended with a macro which modifies
/// the whole function in order to turn it into an async (runtime?)'
/// I haven't dipped too deep into this one yet
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let args: Arguments = argh::from_env();
    let html_directory_path = PathBuf::from(&args.html_folder);

    let (pwm_led_sender, pwm_led_receiver) = channel::<PwmLedMode>();
    let (matrix_display_sender, matrix_display_receiver) = channel::<MatrixDisplayMode>();

    let pwm_current_state = Arc::new(Mutex::new(PwmLedMode::Off));

    let shared_state = AppState {
        pwm_led_sender,
        pwm_current_state: pwm_current_state.clone(),
        buzzer_resource_lock: Arc::new(Mutex::new(())),
        camera_latest_image: Arc::new(Mutex::new(String::from("example.jpg"))),
        matrix_sender: matrix_display_sender,
    };

    // thread to handle led at pwm0
    thread::spawn(move || drive_pwm_led(pwm_current_state.clone(), pwm_led_receiver));
    // thread to handle 8x8 LED display
    thread::spawn(move || drive_matrix_display(matrix_display_receiver));

    let app = app::initialize_app(shared_state, &html_directory_path);

    if args.no_tls {
        start_without_tls(&args, app).await;
    } else {
        start_with_tls(&args, app).await;
    }
}

async fn start_with_tls(args: &Arguments, app: Router) {
    println!("starting server with tls.");
    let tls_config = RustlsConfig::from_pem_file(
        format!("./{}/cert.pem", args.tls_dir),
        format!("./{}/key.pem", args.tls_dir),
    )
    .await
    .unwrap_or_else(|_| {
        panic!(
            "error while loading tls certificate `cert.pem` and key `key.pem` in directory `{}`.",
            &args.tls_dir
        )
    });

    let portnumber = args.portnumber;
    let addr = SocketAddr::from(([0, 0, 0, 0], portnumber as u16));
    println!("Starting TcpListener at {}", &addr);
    // Start the server
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Could not start server.");
}

async fn start_without_tls(args: &Arguments, app: Router) {
    let addr = format!("0.0.0.0:{}", &args.portnumber);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("could not create TcpListener.");
    println!("starting listener at port: {}", &addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("could not serve website.");
}
