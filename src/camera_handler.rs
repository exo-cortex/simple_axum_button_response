use axum::Json;
use serde::Deserialize;

use crate::system_calls;

const GOOD_IMAGE_QUALITY: u8 = 95;
const SHITTY_IMAGE_QUALITY: u8 = 5;

#[derive(Debug, Deserialize)]
pub struct CameraParams {
    camera_signal: Option<String>,
}

pub async fn camera_signals_handler(
    // State(state): State<AppState>,
    Json(web_response): Json<CameraParams>,
) {
    match web_response.camera_signal.as_deref() {
        Some("good") => {
            println!("camera button press detected, attempting to use camera...");
            let output = system_calls::update_image(GOOD_IMAGE_QUALITY);
            println!("done. status: {}", output);
        }

        Some("shitty") => {
            println!("camera button press detected, attempting to use camera...");
            let output = system_calls::update_image(SHITTY_IMAGE_QUALITY);
            println!("done. status: {}", output);
        }
        Some(something_else) => {
            println!("received unexpected camera signal: {}", something_else)
        }
        None => {}
    };
}
