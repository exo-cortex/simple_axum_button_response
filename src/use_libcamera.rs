use std::process::Command;

use std::str;

use chrono::Utc;

const CAMERA_MODES: [(u16, u16); 4] = [(640, 480), (1296, 972), (1920, 1080), (2592, 1944)]; // ZeroCam modes

pub fn update_image(quality: u8) -> String {
    let now = Utc::now();

    println!("{}", now.format("%Y_%m_%d-%H%M%S taking new impage"));

    let (width, height) = CAMERA_MODES[1];

    match Command::new("libcamera-still")
        .current_dir("/home/nt_user/webserver/img/")
        .args([
            "-n",    // do not show image on desktop (is running headless anyway)
            "-t 50", // timeout value in ms
            &format!("--width={}", width),
            &format!("--height={}", height),
            &format!("-q{}", quality),
            "-oexample.jpg", // only works correctly without spaces
        ])
        .output()
    {
        Ok(output) => {
            let bash_output = &output.stdout;

            let s: String = match str::from_utf8(&bash_output) {
                Ok(v) => v.into(),
                Err(e) => format!("invalid UTF-8 sequence: {}", e),
            };

            format!("{}", s)
        }
        Err(e) => format!("could not run command: {:?}", e),
    }
}
