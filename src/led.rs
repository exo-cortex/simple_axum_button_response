pub mod on_off_led;
pub mod pwm_led;

use crate::{led::on_off_led::LedMode, AppState, PwmLedMode};

use {
    axum::{
        extract::{Json, State},
        response::IntoResponse,
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct LedParams {
    pub led_id: Option<String>,
    pub led_signal: Option<String>,
}

// please refactor
pub async fn led_signals_handler(
    State(state): State<AppState>,
    Json(response): Json<LedParams>,
) -> impl IntoResponse {
    println!(
        "led_signals_handler: received signal: {:?}, {:?}",
        &response.led_id, &response.led_signal
    );

    // not everything is handled cleanly
    // please revise
    let response = match response.led_id.as_deref() {
        Some("led") => {
            match response.led_signal.as_deref() {
                Some("off") => {
                    let send_signal = LedMode::Off;
                    state.sender.send(send_signal).unwrap();
                }
                Some("on") => {
                    let send_signal = LedMode::On;
                    state.sender.send(send_signal).unwrap();
                }
                Some("blink_equal_250") => {
                    let send_signal = LedMode::BlinkEqualOnOff { period_ms: 250 };
                    state.sender.send(send_signal).unwrap();
                }
                Some(_) => {}
                None => {}
            }
            "led_ok".into()
        }
        Some("pwm_led") => {
            match response.led_signal.as_deref() {
                Some("off") => {
                    let send_signal = PwmLedMode::Off;
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("on") => {
                    let send_signal = PwmLedMode::On;
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("half") => {
                    let send_signal = PwmLedMode::Half;
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("blink_equal_250") => {
                    let send_signal = PwmLedMode::BlinkEqualOnOff { period_ms: 250 };
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("breath") => {
                    let send_signal = PwmLedMode::Breath { period_ms: 1500 };
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("breath_linear") => {
                    let send_signal = PwmLedMode::BreathLinear { period_ms: 2500 };
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("rising_sawtooth") => {
                    let send_signal = PwmLedMode::RisingSawTooth { period_ms: 1500 };
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some("rising_sawtooth_linear") => {
                    let send_signal = PwmLedMode::RisingSawToothLinear { period_ms: 1500 };
                    state.pwm_sender.send(send_signal).unwrap();
                }
                Some(_) => {}
                None => {}
            }
            "pwm_led_ok".into()
        }
        Some(other_id) => {
            format!(
                "unknown_signal: led_id: {:?}, led_signal: {:?}",
                other_id,
                response.led_signal.as_deref()
            )
        }
        None => "unknown_signal".into(),
    };

    response
}
