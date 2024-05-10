use std::process::Command;

use std::str;

use chrono::Utc;

const CAMERA_MODES: [(u16, u16); 4] = [(640, 480), (1296, 972), (1920, 1080), (2592, 1944)];

pub fn update_image() -> String {
    let now = Utc::now();

    println!("{}", now.format("%Y_%m_%d-%H%M%S taking new impage"));
    // let filename = format!("test_{}", now.time());

    let (width, height) = CAMERA_MODES[3];

    let output = Command::new("libcamera-still")
        .current_dir("/home/nt_user/webserver/img/")
        .args([
            "-n",    // do not show image on desktop (is running headless anyway)
            "-t 50", // timeout value in ms
            &format!("--width={}", width),
            &format!("--height={}", height),
            "-oexample.jpg", // only works correctly without spaces
        ])
        .output()
        .expect("failed to execute command");

    let bash_output = &output.stdout;

    let s: String = match str::from_utf8(&bash_output) {
        Ok(v) => v.into(),
        Err(e) => format!("invalid UTF-8 sequence: {}", e),
    };

    format!("{}", s)
}
