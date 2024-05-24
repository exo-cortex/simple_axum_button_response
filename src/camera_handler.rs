use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use std::sync::Arc;

use crate::{use_libcamera, AppState};

const GOOD_IMAGE_QUALITY: u8 = 95;
const SHITTY_IMAGE_QUALITY: u8 = 5;

#[derive(Debug, Deserialize)]
pub struct CameraParams {
    camera_signal: Option<String>,
}

pub async fn camera_signals_handler(
    State(state): State<AppState>,
    Json(web_response): Json<CameraParams>,
) -> impl IntoResponse {
    println!("camera_signals_handler: received signal");
    let camera_in_use = Arc::clone(&state.camera_in_use);
    let mut in_use = camera_in_use
        .lock()
        .expect("could not get lock for ressource \"camera\"");

    let response = if *in_use {
        "camera_busy"
    } else {
        match web_response.camera_signal.as_deref() {
            Some("good") => {
                *in_use = true;
                println!("camera button press detected, attempting to use camera...");
                let output = use_libcamera::update_image(GOOD_IMAGE_QUALITY);
                println!("done. status: {}", output);
                *in_use = false;
                "camera_ready"
            }
            Some("shitty") => {
                *in_use = true;
                println!("camera button press detected, attempting to use camera...");
                let output = use_libcamera::update_image(SHITTY_IMAGE_QUALITY);
                println!("done. status: {}", output);
                *in_use = false;
                "camera_ready"
            }
            Some(something_else) => {
                println!("received unexpected camera signal: {}", something_else);
                "camera_ready"
            }
            None => "camera_unknown_error",
        }
    };
    response
}
