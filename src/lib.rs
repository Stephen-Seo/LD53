#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

mod helpers;
mod sprites;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

#[no_mangle]
fn update() {
    helpers::fill(3);

    unsafe { *DRAW_COLORS = 2 }
    text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 3 }
    }

    blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    text("Press X to blink", 16, 90);

    unsafe { *DRAW_COLORS = 0x312 }
    if helpers::toggle_car_state() {
        blit(
            &sprites::CAR0,
            50,
            40,
            sprites::CAR0_WIDTH,
            sprites::CAR0_HEIGHT,
            sprites::CAR0_FLAGS,
        );
    } else {
        blit(
            &sprites::CAR1,
            50,
            40,
            sprites::CAR1_WIDTH,
            sprites::CAR1_HEIGHT,
            sprites::CAR1_FLAGS,
        );
    }
    if let Some((x, y)) = helpers::draw_mouse() {}
}
