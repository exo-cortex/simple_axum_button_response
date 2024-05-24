// idea:
// button sends post-request to /send_camera and awaits response
// server sends response "taking image"
// client updates status: camera in use and
// awaits server signal: image taken"
// when received client updates status again


async function poll_status() {
    try {
        const response = await fetch('/status', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json'
            },
        });

        if (!response.ok) {
            throw new Error('Failed to get \'/status\'');
        } else {
            console.log("/status -> response ok");
        }

        let resp = await response.text();
        console.log(resp);

    } catch (error) {
        console.error('Error:', error);
    }
}

const camera_busy_color = "#FFBBBB";

async function click_button_camera(image_quality) {
    document.getElementById("camera_status").style.background = 'red';
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
            document.getElementById("camera_status").style.background = 'red';
            setTimeout(myfunction, 2500)
        }

        let resp = await response.text();
        if (resp == "camera_ready") {
            document.getElementById("camera_status").style.background = 'green';
        }
        //  else if (await response.text() == "camera_picture_taken") {            
        //     console.log(await response.text());
        // }

        // Log the response from the server
        console.log(resp);
    } catch (error) {
        console.error('Error:', error);
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
                buzzer_signal: buzzer_signal
            })
        });

        if (!response.ok) {
            throw new Error('Failed to send signal');
        } else {
            console.log("response ok")
        }

        let resp = await response.text();
        console.log(resp);

    } catch (error) {
        console.error('Error:', error);
    }
}


async function send_led_signal(...led_signals) {
    try {
        // Send a POST request to the server endpoint
        const response = await fetch('/send_led', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            // Send the word "example" as JSON data
            body: JSON.stringify({ 
                led_id: led_signals[0],
                led_signal: led_signals[1],
             })
        });

        // Check if the request was successful
        if (!response.ok) {
            throw new Error('Failed to send word');
        }
        
        // Log the response from the server
        console.log(await response.text());
    } catch (error) {
        console.error('Error:', error);
    }
}

