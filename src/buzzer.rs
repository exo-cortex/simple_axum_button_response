use std::{thread, time::Duration};

use {
    axum::{extract::State, response::IntoResponse, Json},
    serde::Deserialize,
};

use crate::AppState;

use rppal::gpio::Gpio;

/// the signal from the client received by `buzzer_signals_handler`
#[derive(Debug, Deserialize)]
pub struct BuzzerParams {
    buzzer_signal: Option<String>,
}

/// the function that handles signals received ad route `/send_buzzer`
/// if the buzzer is not in use it does send a `BuzzerModeSignal` to the thread
/// that controls the buzzer gpio
pub async fn buzzer_signals_handler(
    State(state): State<AppState>,
    Json(signal_from_client): Json<BuzzerParams>,
) -> impl IntoResponse {
    println!("buzzer_signals_handler: received signal");

    let buzzer_in_use = state.buzzer_in_use.clone();
    let mut in_use = buzzer_in_use.lock().unwrap();

    let response;

    if !*in_use {
        *in_use = true;

        let mut my_buzzer_pin = Gpio::new()
            .expect("could not create gpio object")
            .get(3)
            .expect("could not get access to pin 3")
            .into_output_low();

        match signal_from_client.buzzer_signal.as_deref() {
            Some("off") => {
                println!("buzzer received signal \'off\'");
                if my_buzzer_pin.is_set_high() {
                    my_buzzer_pin.set_low();
                }
                response = "buzzer_ok";
            }
            Some("short") => {
                println!("buzzer received signal \'short\'");
                my_buzzer_pin.set_high();
                thread::sleep(Duration::from_millis(250));
                my_buzzer_pin.set_low();
                response = "buzzer_ok";
            }
            Some("double") => {
                println!("buzzer received signal \'double\'");
                let n = 3;

                if n == 1 {
                    my_buzzer_pin.set_high();
                    thread::sleep(Duration::from_millis(50));
                    my_buzzer_pin.set_low();
                    thread::sleep(Duration::from_millis(100));
                } else {
                    my_buzzer_pin.set_high();
                    thread::sleep(Duration::from_millis(50));
                    for _ in 0..n - 1 {
                        my_buzzer_pin.set_low();
                        thread::sleep(Duration::from_millis(100));
                        my_buzzer_pin.set_high();
                        thread::sleep(Duration::from_millis(50));
                    }
                    my_buzzer_pin.set_low();
                }

                response = "buzzer_ok";
            }
            Some(unknown_signal) => {
                println!(
                    "buzzer received unknown signal \'{}\'. switching off buzzer just to be sure",
                    unknown_signal
                );
                if my_buzzer_pin.is_set_high() {
                    my_buzzer_pin.set_low();
                }
                response = "buzzer_unknown_signal";
            }
            None => {
                println!("buzzer received no signal");
                response = "buzzer_unknown_signal";
            }
        };
        *in_use = false;
    } else {
        response = "buzzer_busy";
    }
    drop(in_use);

    response
}
