use crate::sprites::*;
use tinyrand::{Rand, StdRand};

const CAR_ANIM_FRAMES: u8 = 10;
const BOX_INTERP_RATE: f32 = 0.01f32;
const MOVE_RATE: f32 = 1f32;

pub struct World {
    car_state: bool,
    car_frames: u8,
    // sx, sy, ex, ey, interp, interp-type
    // interp-type:
    //  - 0 linear
    //  - 1 squared
    box_pos: Option<(f32, f32, f32, f32, f32, u8)>,
    rand: StdRand,
    street_offset: f32,
    shrubs: [Option<(f32, f32)>; 8],
    // x, state
    // state:
    //   - 0 broken
    //   - 1 fixed
    house: Option<(f32, u8)>,
}

impl World {
    pub fn new() -> World {
        World {
            car_state: false,
            car_frames: 0,
            box_pos: None,
            rand: StdRand::default(),
            street_offset: 0.0f32,
            shrubs: [None, None, None, None, None, None, None, None],
            house: None,
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *crate::GAMEPAD1 };

        self.car_frames += 1;
        if self.car_frames > CAR_ANIM_FRAMES {
            self.car_frames = 0;
            if self.car_state {
                self.car_state = false;
            } else {
                self.car_state = true;
            }
        }

        self.street_offset -= MOVE_RATE;
        if self.street_offset <= -45f32 {
            self.street_offset += 45f32;
        }

        let mut empty_shrub_exists: Option<usize> = None;
        for (idx, shrub) in self.shrubs.iter_mut().enumerate() {
            if let Some((x, _y)) = shrub {
                if *x < -(PLANT_WIDTH as f32) {
                    shrub.take();
                } else {
                    *x -= MOVE_RATE;
                }
            } else {
                empty_shrub_exists = Some(idx);
            }
        }

        if empty_shrub_exists.is_some() && self.rand.next_u16() % 32 == 0 {
            self.shrubs[empty_shrub_exists.unwrap()] =
                Some((180f32, (self.rand.next_u16() % 80 + 60) as f32));
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            *crate::DRAW_COLORS = 0x21;
        }

        crate::rect(-5, 120, 170, 5);

        let mut x = -5 + self.street_offset.round() as i32;
        while x < 170 {
            crate::rect(x, 140, 30, 5);
            x += 45;
        }

        unsafe {
            *crate::DRAW_COLORS = 0x312;
        }

        for shrub in &self.shrubs {
            if let Some((x, y)) = shrub {
                crate::blit(
                    &PLANT,
                    x.round() as i32,
                    y.round() as i32,
                    PLANT_WIDTH,
                    PLANT_HEIGHT,
                    PLANT_FLAGS,
                );
            }
        }

        if self.car_state {
            crate::blit(&CAR0, 10, 103, CAR0_WIDTH, CAR0_HEIGHT, CAR0_FLAGS);
        } else {
            crate::blit(&CAR1, 10, 103, CAR1_WIDTH, CAR1_HEIGHT, CAR1_FLAGS);
        }
    }
}
