use std::{
    sync::mpsc::{Receiver, RecvTimeoutError},
    time::{Duration, Instant},
};

use rppal::gpio::{Gpio, OutputPin};

pub enum LedMode {
    Off,
    On,
    BlinkEqualOnOff { period_ms: u64 },
}

pub fn drive_led(led_control_receiver: Receiver<LedMode>) {
    println!("hello from led control thread");

    let mut my_led_pin: OutputPin = Gpio::new()
        .expect("could not create gpio object")
        .get(2)
        .expect("could not get access to pin 2")
        .into_output_low();

    let mut pin_state = false;
    let mut led_state = LedMode::Off;
    let mut next_blink: Option<Instant> = None;

    loop {
        let msg = if let Some(next_blink) = next_blink {
            // we're currently blinking the LED, so we have to block for
            // - the next message
            // - or until we need to switch the LED state again
            // whichever comes first
            led_control_receiver.recv_timeout(next_blink.saturating_duration_since(Instant::now()))
            // if nightly:
            // #![feature(deadline_api)]
            // led_control_receiver.recv_deadline(next_blink)
        } else {
            // we're not currently blinking so we can just block
            // until we receive another message
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
