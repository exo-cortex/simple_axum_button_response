use std::process::Command;

use chrono::Utc;

// use this maybe?
// pub struct CameraMode {
//     width: u16,
//     height: u16,
//     quality: u8,
//     timeout: u16,
// }

const CAMERA_MODES: [(u16, u16); 4] = [(640, 480), (1296, 972), (1920, 1080), (2592, 1944)]; // ZeroCam modes

pub async fn take_image(quality: u8) -> (String, String) {
    let now = Utc::now();

    let filename = format!("{}.jpg", now.format("%Y_%m_%d-%H%M%S"));
    println!("taking new image: {}", filename);

    let (width, height) = CAMERA_MODES[1];

    match Command::new("libcamera-still")
        .current_dir("/home/nt_user/webserver/html/img/")
        .args([
            "-n",    // do not show image on desktop (is running headless anyway)
            "-t 20", // timeout value in ms
            &format!("--width={}", width),
            &format!("--height={}", height),
            &format!("-q{}", quality),
            &format!("-o{}", filename), // only works correctly without spaces
        ])
        .output()
    {
        Ok(output) => {
            println!("status: {:?}", output);
            ("camera_ok".into(), filename)
        }
        Err(e) => {
            println!("status: {:?}", e);
            ("camera_error".into(), "example.jpg".into())
        }
    }
}
