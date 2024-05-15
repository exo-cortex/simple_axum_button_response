use crate::AppState;

use std::{
    sync::mpsc::{Receiver, RecvTimeoutError},
    time::{Duration, Instant},
};

use axum::{extract::State, Json};
use rppal::gpio::Gpio;
use serde::Deserialize;

pub enum LedMode {
    Off,
    On,
    BlinkEqualOnOff { period_ms: u64 },
}

#[derive(Debug, Deserialize)]
pub struct LedParams {
    led_signal: Option<String>,
}

pub async fn led_signals_handler(State(state): State<AppState>, Json(response): Json<LedParams>) {
    match response.led_signal.as_deref() {
        Some("off") => {
            state.sender.send(LedMode::Off).unwrap();
        }
        Some("on") => {
            state.sender.send(LedMode::On).unwrap();
        }
        Some("blink_equal_250") => {
            state
                .sender
                .send(LedMode::BlinkEqualOnOff { period_ms: 250 })
                .unwrap();
        }
        Some("blink_equal_100") => {
            state
                .sender
                .send(LedMode::BlinkEqualOnOff { period_ms: 100 })
                .unwrap();
        }
        Some(something_else) => println!("received unexpected signal: {}", something_else),
        None => {
            // println!("camera_button signal received but contained nothing.")
        }
    }
}

pub fn drive_led(led_control_receiver: Receiver<LedMode>) {
    println!("hello from led control thread");

    let mut my_led_pin = Gpio::new()
        .expect("could not create gpio object")
        .get(2)
        .expect("could not get access to pin 2")
        .into_output_low();

    let mut pin_state = false; // later will be the actual GPIO pin
    let mut led_state = LedMode::Off;
    let mut next_blink: Option<Instant> = None;

    loop {
        let msg = if let Some(next_blink) = next_blink {
            led_control_receiver.recv_timeout(next_blink.saturating_duration_since(Instant::now()))
        } else {
            led_control_receiver.recv().map_err(RecvTimeoutError::from)
        };
        match msg {
            Ok(new_led_state) => {
                led_state = new_led_state;
            }
            Err(RecvTimeoutError::Disconnected) => {
                break; // break out of loop
            }
            Err(RecvTimeoutError::Timeout) => {}
        }

        match led_state {
            LedMode::Off => {
                pin_state = false;
                next_blink = None;
            }

            LedMode::On => {
                pin_state = true;
                next_blink = None;
            }

            LedMode::BlinkEqualOnOff { period_ms } => {
                pin_state = !pin_state;
                *next_blink.get_or_insert_with(Instant::now) +=
                    Duration::from_millis(period_ms / 2);
            }
        }
        if pin_state {
            my_led_pin.set_high();
        } else {
            my_led_pin.set_low();
        }
    }
}
