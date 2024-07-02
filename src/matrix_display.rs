use {
    axum::{extract::State, response::IntoResponse, Json},
    max7219::MAX7219,
    rppal::spi::{Bus, Mode, SlaveSelect, Spi},
    serde::Deserialize,
    std::{
        cmp::min,
        sync::mpsc::{Receiver, RecvTimeoutError},
        time::{Duration, Instant},
    },
};

use crate::app::AppState;

#[derive(Debug, Deserialize)]
pub enum MatrixDisplayMode {
    // in JSON:
    AllOff,         // is:   "{ "AllOff" : null }"
    AllOn,          // is:   "{ "AllOn" : null }"
    Brightness(u8), // e.g.: "{ "Brightness" : 5 }"
    Animation,      //       "{ "Animation" : null }"
}

/// handles the matrix display signals
pub async fn matrix_display_signals_handler(
    State(state): State<AppState>,
    Json(display_instruction): Json<MatrixDisplayMode>,
) -> impl IntoResponse {
    println!(
        "received message for matrix display: \n {:#?}",
        &display_instruction
    );

    state.matrix_sender.send(display_instruction).unwrap();
}

/// controls the matrix display and reacts new messages
pub fn drive_matrix_display(matrix_display_receiver: Receiver<MatrixDisplayMode>) {
    println!("hello from matrix display control thread");

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0)
        .expect("cannot initialize SPI module");

    let mut max7219 = MAX7219::from_spi(4, spi).expect("could not initialize display");

    max7219
        .power_on()
        .expect("could not power on max7219 module");
    (0..4).for_each(|i| {
        max7219.clear_display(i).unwrap();
        max7219.set_intensity(i, 0x0F).unwrap();
    });

    let mut next_update_event: Option<Instant> = None;
    let mut display_state = MatrixDisplayMode::AllOff;
    let mut animation_index: i8 = 0;
    let mut animation_inc = 1;

    loop {
        let msg = if let Some(next_update_event) = next_update_event {
            matrix_display_receiver
                .recv_timeout(next_update_event.saturating_duration_since(Instant::now()))
        } else {
            matrix_display_receiver
                .recv()
                .map_err(RecvTimeoutError::from)
        };

        match msg {
            Ok(new_display_state) => {
                display_state = new_display_state;
            }
            Err(RecvTimeoutError::Disconnected) => {
                max7219.power_off().unwrap();
                break;
            }
            Err(RecvTimeoutError::Timeout) => {}
        }

        {
            // mutex stuff ?
        }

        match display_state {
            MatrixDisplayMode::AllOff => {
                max7219.write_raw(0, &[0u8; 8]).unwrap();
                next_update_event = None;
            }
            MatrixDisplayMode::AllOn => {
                max7219.write_raw(0, &[u8::MAX; 8]).unwrap();
                next_update_event = None;
            }
            MatrixDisplayMode::Brightness(value) => {
                max7219.set_intensity(0, min(value, 0x0F)).unwrap();
                next_update_event = None;
            }
            MatrixDisplayMode::Animation => {
                animation_index += animation_inc;
                if animation_index == 7 || animation_index == 0 {
                    animation_inc = -animation_inc;
                };
                max7219
                    .write_raw(0, &IMAGE_DATA_FULL[animation_index as usize])
                    .unwrap();
                *next_update_event.get_or_insert_with(Instant::now) += Duration::from_millis(120);
            }
        }
    }
}

const IMAGE_DATA_FULL: [[u8; 8]; 8] = [
    [
        0b0000_0001,
        0b0000_0010,
        0b0000_0100,
        0b0000_1000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1000_0000,
    ],
    [
        0b0000_0010,
        0b0000_0111,
        0b0000_1110,
        0b0001_1100,
        0b0011_1000,
        0b0111_0000,
        0b1110_0000,
        0b0100_0000,
    ],
    [
        0b0000_0100,
        0b0000_1110,
        0b0001_1111,
        0b0011_1110,
        0b0111_1100,
        0b1111_1000,
        0b0111_0000,
        0b0010_0000,
    ],
    [
        0b0000_1000,
        0b0001_1100,
        0b0011_1110,
        0b0111_1111,
        0b1111_1110,
        0b0111_1100,
        0b0011_1000,
        0b0001_0000,
    ],
    [
        0b0001_0000,
        0b0011_1000,
        0b0111_1100,
        0b1111_1110,
        0b0111_1111,
        0b0011_1110,
        0b0001_1100,
        0b0000_1000,
    ],
    [
        0b0010_0000,
        0b0111_0000,
        0b1111_1000,
        0b0111_1100,
        0b0011_1110,
        0b0001_1111,
        0b0000_1110,
        0b0000_0100,
    ],
    [
        0b0100_0000,
        0b1110_0000,
        0b0111_0000,
        0b0011_1000,
        0b0001_1100,
        0b0000_1110,
        0b0000_0111,
        0b0000_0010,
    ],
    [
        0b1000_0000,
        0b0100_0000,
        0b0010_0000,
        0b0001_0000,
        0b0000_1000,
        0b0000_0100,
        0b0000_0010,
        0b0000_0001,
    ],
];
