const resource_busy_color = "#FFBBBB";
const resource_free_color = "#BBFFBB";
const resource_error_color = "#FFFFBB";

function poll_status() {
    update_state(false);
    setTimeout(poll_status, 1000);
}


async function update_state(do_logging) {
    const url = '/status';
    try {
        const response = await fetch(url, {
            method: 'GET',
        });
        const json = await response.json();
        
        if (do_logging) {
            console.log(json);
        }

        if (json.buzzer_status == 'buzzer_free') {
            document.getElementById('buzzer_status').style.background = resource_free_color;
        } else if (json.buzzer_status == 'buzzer_busy') {
            document.getElementById('buzzer_status').style.background = resource_busy_color;
        } else {}

        if (json.latest_image != "img/example.jpg") {
            document.getElementById("latest_image").src = 'img/' + json.latest_image;
        }

        if (json.camera_status == 'camera_ok') {
            document.getElementById("camera_status").style.background = resource_free_color;
        } else {
            document.getElementById("camera_status").style.background = resource_busy_color;
        }
        // } else if (json.camera_status == "camera_busy") {

    } catch (error) {
        console.error('Error:', error)
    }
}

async function click_button_buzzer(buzzer_signal) {
    try {
        const response = await fetch("/send_buzzer", {
            method: 'POST',
            headers: {
                // 'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                buzzer_instruction: buzzer_signal
            })
        });

        if (!response.ok) {
            throw new Error('Failed to send signal');
        } else {
            document.getElementById('buzzer_status').style.background = resource_free_color;
            console.log("/send_buzzer: response: ok")
        }

        let json_resp = await response.json();
        // console.log(json_resp);
        // if (json_resp.buzzer_status == 'buzzer_free') {
        //     document.getElementById('buzzer_status').style.background = resource_free_color;
        // } else 
        if (json_resp.buzzer_status == 'buzzer_busy') {
            document.getElementById('buzzer_status').style.background = resource_busy_color;
        } else {}

    } catch (error) {
        console.error('Error:', error);
    }
}

async function pwm_led(pwm_led_signal) {
    try {
        const response = await fetch("/send_led", {
            method: 'POST',
            headers: {
                // 'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                led_id: "pwm_led",
                led_signal: pwm_led_signal,
            })
        });

        if (!response.ok) {
            throw new Error('Failed to send signal');
        } else {
            // document.getElementById('led_status').style.background = resource_free_color;
            // console.log("/send_led: response: ok")
        }

        let json_resp = await response.json();
        // console.log(json_resp);
        if (json_resp.buzzer_status == 'buzzer_free') {
            document.getElementById('buzzer_status').style.background = resource_free_color;
        } else 
        if (json_resp.buzzer_status == 'buzzer_busy') {
            document.getElementById('buzzer_status').style.background = resource_busy_color;
        } else {}

    } catch (error) {
        console.error('Error:', error);
    }
}

async function matrix_off() {
    let json_body = JSON.stringify({ 'AllOff': null });
    matrix_display_on(json_body)
}

async function matrix_on() {
    let json_body = JSON.stringify({ 'AllOn': null });
    matrix_display_on(json_body)
}

async function matrix_brightness(value) {
    let json_body = JSON.stringify({ 'Brightness': value });
    matrix_display_on(json_body)
}

async function matrix_animation() {
    let json_body = JSON.stringify({ 'Animation': null });
    matrix_display_on(json_body)
}

async function matrix_display_on(json_body) {
    try {
        const response = await fetch("/send_matrix_display", {
            method: 'POST',
            headers: {
                // 'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: json_body
        });

        if (!response.ok) {
            throw new Error('Failed to send signal');
        } else {

        }
        // let json_resp = await response.json();
    } catch (error) {
        console.error('Error:', error);
    }
}

async function click_button_camera(image_quality) {
    document.getElementById("camera_status").style.background = resource_busy_color;
    // document.getElementById("camera_status").style.background = 'red';
    try {
        // Send a POST request to the server endpoint
        const response = await fetch('/send_camera', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            // Send the word "example" as JSON data
            body: JSON.stringify({ camera_signal: image_quality })
        });


        // Check if the request was successful
        if (!response.ok) {
            throw new Error('Failed to send word');
        } else {
            console.log("response ok")
        }
        
        let json_resp = await response.json();
        console.log(json_resp);

        if (json_resp.lib_camera_status == 'camera_ok') {
            document.getElementById('camera_status').style.background = resource_free_color;
            // let new_image_location = json_resp.latest_image;
            // console.log(new_image_location);
            // console.log(String('img/',json_resp.latest_image));
            document.getElementById('latest_image').src = 'img/' + json_resp.latest_image;
            console.log("camera_response_json received.");
        }
        //  else if (json_resp.lib_camera_status == 'camera_error') {
        //     document.getElementById('camera_status').style.background = resource_error_color;
        //     document.getElementById('latest_image').src = 'img/' + json_resp.latest_image;
        //     console.log("camera_response_json received.");
        // }

    } catch (error) {
        console.error('Error:', error);
    }
}

poll_status();