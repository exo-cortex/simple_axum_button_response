use axum::{routing::get_service, Router};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn routes_static(path: &PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(path.as_os_str())))
}
