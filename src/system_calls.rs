use std::process::Command;

use std::str;

use chrono::Utc;

pub fn update_image() -> String {
    let now = Utc::now();

    // let filename = format!("test_{}", now.time());

    // let (width, height) = (640, 480);
    // let fullargs = format!(
    //     "--width {} --height {} -o {} ",
    //     width, height, "~/webserver/img/example.jpg"
    // );
    // let resolution = format!("--width {} --height {} -o {}", width, height);
    // let image_file = format!("-o ~/webserver/img/example.jpg");

    let output = Command::new("libcamera-still")
        .current_dir("/home/nt_user/webserver/img/")
        .args([
            "-n",    // do not show image on desktop (is running headless anyway)
            "-t 50", // timeout value
            "--width=480",
            "--height=300",
            "-oexample.jpg", // only works correctly without spaces
        ])
        .output()
        .expect("failed to execute command");

    let bash_output = output.stdout;

    let s: String = match str::from_utf8(&bash_output) {
        Ok(v) => v.into(),
        Err(e) => format!("invalid UTF-8 sequence: {}", e),
    };

    format!("{}", s)
}
