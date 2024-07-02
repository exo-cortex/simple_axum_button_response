use {
    axum::{extract::State, response::IntoResponse, Json},
    serde::{Deserialize, Serialize},
};

use crate::{app::AppState, use_libcamera};

const GOOD_IMAGE_QUALITY: u8 = 95;
const SHITTY_IMAGE_QUALITY: u8 = 5;

#[derive(Debug, Deserialize)]
pub struct CameraParams {
    camera_signal: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CameraHandlerResponse {
    lib_camera_status: String,
    latest_image: String,
}

// todo: remove `CameraParams` and use a CameraMsg-enum directly.
pub async fn camera_signals_handler(
    State(state): State<AppState>,
    Json(client_message): Json<CameraParams>,
) -> impl IntoResponse {
    println!("camera_signals_handler: received signal");
    let cloned_arc_camera_latest_image = state.camera_latest_image.clone();

    let command_output: String;
    let mut new_filename: String = String::from("");

    match cloned_arc_camera_latest_image.try_lock() {
        Ok(mut latest_image) => match client_message.camera_signal.as_deref() {
            Some("good") => {
                println!("camera button press detected, attempting to use camera...");
                (command_output, new_filename) =
                    use_libcamera::take_image(GOOD_IMAGE_QUALITY).await;
                latest_image.clone_from(&new_filename);
            }
            Some("shitty") => {
                println!("camera button press detected, attempting to use camera...");
                (command_output, new_filename) =
                    use_libcamera::take_image(SHITTY_IMAGE_QUALITY).await;
                latest_image.clone_from(&new_filename);
            }
            Some(something_else) => {
                command_output = "camera_unknown_signal".into();
                println!("received unexpected camera signal: {}", something_else);
            }
            None => {
                command_output = "camera_unknown_signal".into();
            }
        },
        Err(_) => {
            command_output = "camera_busy".into();
        }
    }

    Json(CameraHandlerResponse {
        lib_camera_status: command_output,
        latest_image: new_filename,
    })
}
