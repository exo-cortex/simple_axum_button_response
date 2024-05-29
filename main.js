const camera_busy_color = "#FFBBBB";
const camera_free_color = "#BBFFBB";





function poll_status() {
    get_status(false);
    setTimeout(poll_status, 500);
}

async function get_status(do_logging) {
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
            document.getElementById('buzzer_status').style.background = camera_free_color;
        } else if (json.buzzer_status == 'buzzer_busy') {
            document.getElementById('buzzer_status').style.background = camera_busy_color;
        } else {}

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
            document.getElementById('buzzer_status').style.background = camera_free_color;
            console.log("/send_buzzer: response: ok")
        }

        let json_resp = await response.json();
        // console.log(json_resp);
        // if (json_resp.buzzer_status == 'buzzer_free') {
        //     document.getElementById('buzzer_status').style.background = camera_free_color;
        // } else 
        if (json_resp.buzzer_status == 'buzzer_busy') {
            document.getElementById('buzzer_status').style.background = camera_busy_color;
        } else {}

    } catch (error) {
        console.error('Error:', error);
    }
}

poll_status();