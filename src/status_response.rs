use crate::app::AppState;

use axum::{
    extract::{ConnectInfo, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;

/// the status-response which is serialized into json

#[derive(Serialize)]
pub struct StatusResponse {
    buzzer_status: String,
    latest_image: String,
    camera_status: String,
    server_status: String,
}

/// sorry - very ugly
/// the function which handles status responses
pub async fn status_response(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let cloned_buzzer_in_use = state.buzzer_resource_lock.clone();
    let busy = cloned_buzzer_in_use.try_lock().is_err();

    let cloned_camera_latest_image = state.camera_latest_image.clone();
    let (latest_image, camera_status) = match cloned_camera_latest_image.try_lock() {
        Ok(image) => (image.clone(), "camera_ok".into()),
        Err(_) => ("img/example.jpg".into(), "camera_busy".into()),
    };

    println!("\'get\' request to \'/status\' from {}", addr);

    Json(StatusResponse {
        buzzer_status: if busy {
            "buzzer_busy".into()
        } else {
            "buzzer_free".into()
        },
        latest_image,
        camera_status,
        server_status: "online".into(),
    })
}
