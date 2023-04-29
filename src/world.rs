use crate::helpers::round_f32_to_i32;
use crate::sprites::*;
use tinyrand::{Rand, StdRand};

const CAR_ANIM_FRAMES: u8 = 10;
const MOVE_RATE: f32 = 1f32;
const MULTIPLIER_INC_RATE: f32 = 0.13f32;
const HOUSE_FRAMES: u32 = 100;
const HOUSE_RANGE: f32 = 90f32;

pub struct World {
    car_state: bool,
    car_frames: u8,
    rand: StdRand,
    street_offset: f32,
    shrubs: [Option<(f32, f32)>; 8],
    // x, state
    // state:
    //   - 0 broken
    //   - 1 fixed
    house: Option<(f32, u8)>,
    house_frames: u32,
    house_frames_max: u32,
    is_in_range: bool,
    score: u64,
    rate_multiplier: f32,
    status_text: Option<(&'static str, u32)>,
    str_buf: [u8; 16],
}

impl World {
    pub fn new() -> World {
        World {
            car_state: false,
            car_frames: 0,
            rand: StdRand::default(),
            street_offset: 0.0f32,
            shrubs: [None, None, None, None, None, None, None, None],
            house: None,
            house_frames: 0,
            house_frames_max: HOUSE_FRAMES,
            is_in_range: false,
            score: 0,
            rate_multiplier: 1f32,
            status_text: None,
            str_buf: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn get_move_rate(&self) -> f32 {
        MOVE_RATE * self.rate_multiplier
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *crate::GAMEPAD1 };
        let mouse = unsafe { *crate::MOUSE_BUTTONS };

        self.car_frames += 1;
        if self.car_frames > CAR_ANIM_FRAMES {
            self.car_frames = 0;
            if self.car_state {
                self.car_state = false;
            } else {
                self.car_state = true;
            }
        }

        self.street_offset -= self.get_move_rate();
        if self.street_offset <= -45f32 {
            self.street_offset += 45f32;
        }

        let mut empty_shrub_exists: Option<usize> = None;
        let move_rate = self.get_move_rate();
        for (idx, shrub) in self.shrubs.iter_mut().enumerate() {
            if let Some((x, _y)) = shrub {
                if *x < -(PLANT_WIDTH as f32) {
                    shrub.take();
                } else {
                    *x -= move_rate;
                }
            } else {
                empty_shrub_exists = Some(idx);
            }
        }

        if empty_shrub_exists.is_some() && self.rand.next_u16() % 32 == 0 {
            self.shrubs[empty_shrub_exists.unwrap()] =
                Some((180f32, (self.rand.next_u16() % 80 + 60) as f32));
        }

        if self.house.is_none() {
            self.house_frames += 1;
            if self.house_frames > self.house_frames_max {
                self.house_frames = 0;
                self.house = Some((170f32, 0));
                self.house_frames_max = HOUSE_FRAMES + (self.rand.next_u16() % 100) as u32;
            }
        } else {
            self.house.as_mut().unwrap().0 -= self.get_move_rate();
            let pos_ref: &f32 = &self.house.as_ref().unwrap().0;
            let state_ref: &u8 = &self.house.as_ref().unwrap().1;
            if *state_ref == 0 && *pos_ref < HOUSE_RANGE && *pos_ref >= -20f32 {
                self.is_in_range = true;
            } else if *pos_ref < -(HOUSE0_WIDTH as f32) {
                if self.is_in_range {
                    self.rate_multiplier /= 2f32;
                    if self.rate_multiplier < 1f32 {
                        self.rate_multiplier = 1f32;
                    }
                    self.status_text = Some(("Slow down!", 80));
                }
                self.house.take();
                self.is_in_range = false;
            } else {
                if self.is_in_range {
                    self.rate_multiplier /= 2f32;
                    if self.rate_multiplier < 1f32 {
                        self.rate_multiplier = 1f32;
                    }
                    self.status_text = Some(("Slow down!", 80));
                }
                self.is_in_range = false;
            }
        }

        if self.is_in_range
            && ((gamepad & crate::BUTTON_1) != 0 || (mouse & crate::MOUSE_LEFT) != 0)
        {
            self.is_in_range = false;
            self.house.as_mut().unwrap().1 = 1;
            self.score += 1;
            self.rate_multiplier += MULTIPLIER_INC_RATE;
            self.status_text = Some(("Speed up!", 80));
        }

        if let Some((_, t)) = &mut self.status_text {
            *t -= 1;
            if *t == 0 {
                self.status_text.take();
            }
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            *crate::DRAW_COLORS = 0x21;
        }

        crate::rect(-5, 120, 170, 5);

        let mut x = -5 + round_f32_to_i32(self.street_offset);
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
                    round_f32_to_i32(*x),
                    round_f32_to_i32(*y),
                    PLANT_WIDTH,
                    PLANT_HEIGHT,
                    PLANT_FLAGS,
                );
            }
        }

        if let Some((x, state)) = self.house {
            match state {
                0 => crate::blit(
                    &HOUSE1,
                    round_f32_to_i32(x),
                    30 + HOUSE0_HEIGHT as i32 - HOUSE1_HEIGHT as i32,
                    HOUSE1_WIDTH,
                    HOUSE1_HEIGHT,
                    HOUSE1_FLAGS,
                ),
                1 => crate::blit(
                    &HOUSE0,
                    round_f32_to_i32(x),
                    30,
                    HOUSE0_WIDTH,
                    HOUSE0_HEIGHT,
                    HOUSE0_FLAGS,
                ),
                _ => (),
            }
        }

        if self.car_state {
            crate::blit(&CAR0, 10, 103, CAR0_WIDTH, CAR0_HEIGHT, CAR0_FLAGS);
        } else {
            crate::blit(&CAR1, 10, 103, CAR1_WIDTH, CAR1_HEIGHT, CAR1_FLAGS);
        }

        if self.is_in_range {
            unsafe {
                *crate::DRAW_COLORS = 0x1;
            }
            crate::text("Tap or Press X!", 5, 5);
        }

        unsafe {
            *crate::DRAW_COLORS = 0x1;
        }

        if let Some((s, t)) = self.status_text {
            if (t / 10) % 2 == 0 {
                crate::text(s, 20, 30);
            }
        }

        let mut width = 0;
        let mut temp = self.score;
        while temp > 0 {
            temp /= 10;
            width += 1;
        }
        if width == 0 {
            width = 1;
        }
        temp = self.score;
        if width < 15 {
            for i in 0..width {
                self.str_buf[width - 1 - i] = '0' as u8 + (temp % 10) as u8;
                temp /= 10;
            }
            for i in width..16 {
                self.str_buf[i] = 0;
            }
            crate::custom_text(self.str_buf, width, 160 - width as i32 * 8, 0);
        } else {
            crate::text("99999999999999", 160 - 10 * 8, 0);
        }
    }
}
