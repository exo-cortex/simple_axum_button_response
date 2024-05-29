use {
    axum::{extract::State, response::IntoResponse, Json},
    rppal::gpio::{Gpio, OutputPin},
    serde::{Deserialize, Serialize},
    tokio::time::Duration,
};

use crate::AppState;

/// `buzzer_signals_handler`
#[derive(Debug, Deserialize)]
pub struct BuzzerParams {
    buzzer_instruction: Option<String>,
}

// the thing that `buzzer_signals_handler` sends as a response
#[derive(Debug, Serialize)]
struct BuzzerResponse {
    buzzer_status: String,
}

pub async fn buzzer_signals_handler(
    State(state): State<AppState>,
    Json(client_message): Json<BuzzerParams>,
) -> impl IntoResponse {
    println!("buzzer_signals_handler: received signal");

    let cloned_arc_buzzer_in_use = state.buzzer_resource_lock.clone();

    let response: &str;
    match cloned_arc_buzzer_in_use.try_lock() {
        Ok(_) => {
            response = handle_message(client_message.buzzer_instruction.clone()).await;
        }
        Err(_) => response = "buzzer_busy",
    }

    Json(BuzzerResponse {
        buzzer_status: response.into(),
    })
}

async fn handle_message<'a>(buzzer_instruction: Option<String>) -> &'a str {
    let mut buzzer_pin = Gpio::new()
        .expect("could not create gpio object")
        .get(3)
        .expect("could not get access to pin 3")
        .into_output_low();

    match buzzer_instruction.as_deref() {
        Some("off") => {
            println!("buzzer received signal \'off\'");
            buzzer_pin.set_low();
            "buzzer_ok"
        }
        Some("short") => {
            println!("buzzer received signal \'short\'");
            odd_on_off_buzzer_sequence(&mut buzzer_pin, &[250]).await;
            "buzzer_ok"
        }
        Some("double") => {
            println!("buzzer received signal \'double\'");
            odd_on_off_buzzer_sequence(&mut buzzer_pin, &[100, 200, 100]).await;
            "buzzer_ok"
        }
        Some("silent_2000") => {
            println!("buzzer received signal \'silent_2000\'");
            tokio::time::sleep(Duration::from_millis(2000)).await;
            "buzzer_silent_ok"
        }
        Some(unknown_signal) => {
            println!(
                "buzzer received unknown signal \'{}\'. switching off buzzer just to be sure",
                unknown_signal
            );
            if buzzer_pin.is_set_high() {
                buzzer_pin.set_low();
            }
            "buzzer_unknown_signal"
        }
        None => {
            println!("buzzer received no signal");
            "buzzer_unknown_signal"
        }
    }
}

const MAX_DURATION_MS: u16 = 1500;
const MAX_SEQUENCE_LENGTH: usize = 11;

async fn odd_on_off_buzzer_sequence(pin: &mut OutputPin, sequence: &[u16]) {
    // check if sequence is odd AND not more than MAX_SEQUENCE_LENGTH elements
    if sequence.len() & 0x1 == 0 || sequence.len() > MAX_SEQUENCE_LENGTH {
        return;
    }

    for chunk in sequence.chunks(2) {
        match chunk {
            &[on_ms, off_ms] => {
                let on_ms = if on_ms <= MAX_DURATION_MS {
                    on_ms
                } else {
                    MAX_DURATION_MS
                };
                let off_ms = if off_ms <= MAX_DURATION_MS {
                    off_ms
                } else {
                    MAX_DURATION_MS
                };
                pin.set_high();
                tokio::time::sleep(Duration::from_millis(on_ms as u64)).await;
                pin.set_low();
                tokio::time::sleep(Duration::from_millis(off_ms as u64)).await;
            }
            &[on_ms] => {
                let on_ms = if on_ms <= 1000 { on_ms } else { 1000 };
                pin.set_high();
                tokio::time::sleep(Duration::from_millis(on_ms as u64)).await;
                pin.set_low();
            }
            _ => {
                pin.set_low();
            }
        }
    }
}
