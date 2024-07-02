use {
    rppal::pwm::{Channel::Pwm0, Polarity, Pwm},
    std::{
        f64::consts::TAU,
        sync::{
            mpsc::{Receiver, RecvTimeoutError},
            Arc,
        },
        time::{Duration, Instant},
    },
    tokio::sync::Mutex,
};

const PWM_CYCLE_FREQUENCY_HZ: f64 = 1000.0;
const PWM_UPDATE_FREQUENCY_HZ: f64 = 100.0;

#[derive(Clone)]
pub enum PwmLedMode {
    Off,
    On,
    Half,
    BlinkEqualOnOff { period_ms: u16 },
    Breath { period_ms: u16 },
    BreathLinear { period_ms: u16 },
    Wave { period_ms: u16 },
    WaveLinear { period_ms: u16 },
    RisingSawTooth { period_ms: u16 },
    RisingSawToothLinear { period_ms: u16 },
    FallingSawTooth { period_ms: u16 },
    FallingSawToothLinear { period_ms: u16 },
}

pub fn drive_pwm_led(
    pwm_state: Arc<Mutex<PwmLedMode>>,
    pwm_led_control_receiver: Receiver<PwmLedMode>,
) {
    println!("hello from pwm control thread");

    let pwm_pin = Pwm::with_frequency(Pwm0, 10.0, 0.0, Polarity::Normal, true)
        .expect("could not access Pwm0");

    let mut led_state = PwmLedMode::Off;
    let mut frequency;
    let mut duty_cycle;
    let mut phase = 0.0;
    let mut next_pwm_event: Option<Instant> = None;

    loop {
        let msg = if let Some(next_pwm_event) = next_pwm_event {
            // we're currently blinking the LED, so we have to block for
            // - the next message
            // - or until we need to switch the LED state again
            // whichever comes first
            pwm_led_control_receiver
                .recv_timeout(next_pwm_event.saturating_duration_since(Instant::now()))
            // if nightly:
            // #![feature(deadline_api)]
            // pwm_led_control_receiver.recv_deadline(next_pwm_event)
        } else {
            // we're not currently blinking so we can just block
            // until we receive another message
            pwm_led_control_receiver
                .recv()
                .map_err(RecvTimeoutError::from)
        };
        match msg {
            Ok(new_led_state) => {
                led_state = new_led_state;
            }
            Err(RecvTimeoutError::Disconnected) => {
                pwm_pin
                    .set_frequency(1.0, 0.0)
                    .expect("could not set frequency and duty cycle at Pwm0");
                break; // break out of loop
            }
            Err(RecvTimeoutError::Timeout) => {}
        }

        {
            let pwm_state = pwm_state.clone();
            let mut state = pwm_state.try_lock().unwrap(); //.expect("could not lock pwm_state");
            *state = led_state.clone();
        }

        match led_state {
            PwmLedMode::Off => {
                frequency = 0.5;
                duty_cycle = 0.0;
                next_pwm_event = None;
            }
            PwmLedMode::On => {
                frequency = PWM_CYCLE_FREQUENCY_HZ;
                duty_cycle = 0.5; // let's be safe and use only 50% right now
                next_pwm_event = None;
            }
            PwmLedMode::Half => {
                frequency = PWM_CYCLE_FREQUENCY_HZ;
                duty_cycle = 0.25; // is only 25% because `full` is only 50% right now
                next_pwm_event = None;
            }
            PwmLedMode::BlinkEqualOnOff { period_ms } => {
                frequency = 1000.0 / period_ms as f64;
                duty_cycle = 0.5;
                next_pwm_event = None;
            }
            PwmLedMode::Breath { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                let value = update_phase_map_to_intensity(&mut phase, &period_ms);
                duty_cycle = map_triangle(value);
                duty_cycle *= duty_cycle;
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::BreathLinear { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                let value = update_phase_map_to_intensity(&mut phase, &period_ms);
                duty_cycle = map_triangle(value);
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::Wave { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                let value = update_phase_map_to_intensity(&mut phase, &period_ms);
                duty_cycle = (value * TAU).sin();
                duty_cycle *= duty_cycle;
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::WaveLinear { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                let value = update_phase_map_to_intensity(&mut phase, &period_ms);
                duty_cycle = 0.5 + (value * TAU).sin() * 0.5;
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::RisingSawTooth { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                duty_cycle = update_phase_map_to_intensity(&mut phase, &period_ms);
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::RisingSawToothLinear { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                let value = update_phase_map_to_intensity(&mut phase, &period_ms);
                duty_cycle = value * value;
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::FallingSawTooth { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                duty_cycle = 1.0 - update_phase_map_to_intensity(&mut phase, &period_ms);
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            }
            PwmLedMode::FallingSawToothLinear { period_ms } => {
                frequency = PWM_UPDATE_FREQUENCY_HZ;
                let value = update_phase_map_to_intensity(&mut phase, &period_ms);
                duty_cycle = (1.0 - value) * (1.0 - value);
                *next_pwm_event.get_or_insert_with(Instant::now) +=
                    Duration::from_millis((1000.0 / PWM_UPDATE_FREQUENCY_HZ) as u64);
            } // _ => {
              //     println!("pwm mode unimplemented!")
              // }
        }

        pwm_pin
            .set_frequency(frequency, duty_cycle)
            .expect("could not set frequency and duty cycle at Pwm0");
    }
}

fn update_phase_map_to_intensity(phase: &mut f64, period_ms: &u16) -> f64 {
    *phase += (1000.0 / *period_ms as f64) / PWM_UPDATE_FREQUENCY_HZ;
    phase.rem_euclid(1.0)
}

fn map_triangle(linear_value: f64) -> f64 {
    if linear_value < 0.5 {
        2.0 * linear_value
    } else {
        2.0 * (1.0 - linear_value)
    }
}

fn _map_uneven_triangle(linear_value: f64, midpoint: f64) -> f64 {
    if linear_value < midpoint {
        linear_value / midpoint
    } else {
        (1.0 - linear_value) * (1.0 - midpoint)
    }
}
