use crate::helpers::round_f32_to_i32;
use crate::music::Music;
use crate::sprites::*;
use tinyrand::{Rand, Seeded, StdRand};

const CAR_ANIM_FRAMES: u8 = 10;
const MOVE_RATE: f32 = 1f32;
const MULTIPLIER_INC_RATE: f32 = 0.25f32;
const SPEEDUP_INC: f32 = 0.47f32;
const SLOWDOWN_DIV: f32 = 1.3f32;
const BUILDING_FRAMES: u32 = 100;
const BUILDING_RANGE: f32 = 90f32;
const DAMAGED_FRAMES_MAX: u16 = 120;
const GAMEOVER_SLOWDOWN_RATE: f32 = 0.01f32;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Building {
    House,
    SpeedUp,
    SlowDown,
    WrongHouse,
}

impl Building {
    pub fn random(rand: &mut StdRand) -> Self {
        match rand.next_u16() % 30 {
            0..=9 => Building::House,
            10..=14 => Building::SpeedUp,
            15..=19 => Building::SlowDown,
            20..=29 => Building::WrongHouse,
            _ => unreachable!(),
        }
    }
}

pub struct World {
    car_state: bool,
    car_frames: u8,
    rand: StdRand,
    street_offset: f32,
    shrubs: [Option<(f32, f32)>; 8],
    // x, type, state
    // state:
    //   - 0 broken
    //   - 1 fixed
    building: Option<(f32, Building, u8)>,
    building_frames: u32,
    building_frames_max: u32,
    is_in_range: bool,
    score: u64,
    rate_multiplier: f32,
    status_text: Option<(&'static str, u32)>,
    score_buf: [u8; 16],
    music: Music,
    lives: u8,
    damaged_frames: u16,
}

impl World {
    pub fn new() -> World {
        World {
            car_state: false,
            car_frames: 0,
            rand: StdRand::seed(3),
            street_offset: 0.0f32,
            shrubs: [None, None, None, None, None, None, None, None],
            building: None,
            building_frames: 0,
            building_frames_max: BUILDING_FRAMES,
            is_in_range: false,
            score: 0,
            rate_multiplier: 1f32,
            status_text: Some(("Ludum Dare 53:\nHouse Delivery!\n\nBy: BurnedKirby", 300)),
            score_buf: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            music: Music::new(),
            lives: 3,
            damaged_frames: 0,
        }
    }

    fn reset(&mut self) {
        self.car_state = false;
        self.car_frames = 0;
        self.street_offset = 0f32;
        for shrub in &mut self.shrubs {
            shrub.take();
        }
        self.building = None;
        self.building_frames = 0;
        self.building_frames_max = BUILDING_FRAMES;
        self.is_in_range = false;
        self.score = 0;
        self.rate_multiplier = 1f32;
        self.status_text = Some(("Ludum Dare 53:\nHouse Delivery!\n\nBy: BurnedKirby", 300));
        for score in &mut self.score_buf {
            *score = 0;
        }
        self.music.reset();
        self.lives = 3;
        self.damaged_frames = 0;
    }

    pub fn decrement_lives(&mut self) -> bool {
        if self.lives > 0 {
            self.lives -= 1;
            if self.lives == 0 {
                self.music.gameover();
            }
        }
        self.lives != 0
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

        if self.rate_multiplier <= 0f32 {
            self.car_state = true;
        }

        self.street_offset -= self.get_move_rate();
        if self.street_offset <= -45f32 {
            self.street_offset += 45f32;
        }

        self.music.update();

        if let Some((_, t)) = &mut self.status_text {
            *t -= 1;
            if *t == 0 {
                self.status_text.take();
            }
        }

        if self.damaged_frames != 0 {
            self.damaged_frames -= 1;
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

        {
            let move_rate = self.get_move_rate();
            if let Some((x, _, _)) = &mut self.building {
                *x -= move_rate;
            }
        }

        if self.lives == 0 {
            self.rate_multiplier -= GAMEOVER_SLOWDOWN_RATE;
            if self.rate_multiplier < 0f32 {
                self.rate_multiplier = 0f32;
            }
            if (gamepad & crate::BUTTON_2) != 0 {
                self.reset();
            }
            return;
        }

        if empty_shrub_exists.is_some() && self.rand.next_u16() % 32 == 0 {
            self.shrubs[empty_shrub_exists.unwrap()] =
                Some((180f32, (self.rand.next_u16() % 80 + 60) as f32));
        }

        if self.building.is_none() {
            self.building_frames += 1;
            if self.building_frames > self.building_frames_max {
                self.building_frames = 0;
                self.building = Some((170f32, Building::random(&mut self.rand), 0));
                self.building_frames_max = BUILDING_FRAMES + (self.rand.next_u16() % 100) as u32;
            }
        } else {
            let pos_ref: &f32 = &self.building.as_ref().unwrap().0;
            let building_type = self.building.as_ref().unwrap().1;
            let state_ref: &u8 = &self.building.as_ref().unwrap().2;
            if *state_ref == 0 && *pos_ref < BUILDING_RANGE && *pos_ref >= -20f32 {
                self.is_in_range = true;
            } else if building_type == Building::House && *pos_ref < -(HOUSE0_WIDTH as f32) {
                if self.is_in_range {
                    if self.decrement_lives() {
                        self.rate_multiplier /= 2f32;
                        if self.rate_multiplier < 1f32 {
                            self.rate_multiplier = 1f32;
                        }
                        self.status_text = Some(("Miss!\nSlow down!", 120));
                        self.music.slow_down();
                        self.damaged_frames = DAMAGED_FRAMES_MAX;
                    }
                    self.music.damaged();
                }
                self.building.take();
                self.is_in_range = false;
            } else if (building_type == Building::SpeedUp || building_type == Building::SlowDown)
                && *pos_ref < -(SPEEDUP_WIDTH as f32)
            {
                self.building.take();
                self.status_text = Some(("It's OK!\nKeep delivering!", 120));
                self.is_in_range = false;
            } else if building_type == Building::WrongHouse && *pos_ref < -(HOUSE0_WIDTH as f32) {
                if self.is_in_range {
                    self.status_text = Some(("OK!\nKeep going!", 120));
                }
                self.building.take();
                self.is_in_range = false;
            }
        }

        if (gamepad & crate::BUTTON_1) != 0 {
            self.rand.next_u16();
            if self.is_in_range {
                self.is_in_range = false;
                match self.building.as_ref().unwrap().1 {
                    Building::House => {
                        self.building.as_mut().unwrap().2 = 1;
                        self.score += 1;
                        self.rate_multiplier += MULTIPLIER_INC_RATE;
                        self.status_text = Some(("Nice delivery!\nSpeed up!", 120));
                        self.music.speed_up();
                    }
                    Building::SpeedUp => {
                        self.rate_multiplier += SPEEDUP_INC;
                        self.status_text = Some(("Speed up!", 120));
                        self.building.take();
                        self.music.speed_up();
                    }
                    Building::SlowDown => {
                        self.rate_multiplier /= SLOWDOWN_DIV;
                        if self.rate_multiplier < 1f32 {
                            self.rate_multiplier = 1f32;
                        }
                        self.status_text = Some(("Slow down!", 120));
                        self.building.take();
                        self.music.slow_down();
                    }
                    Building::WrongHouse => {
                        self.building.as_mut().unwrap().2 = 1;
                        if self.decrement_lives() {
                            self.rate_multiplier /= SLOWDOWN_DIV;
                            if self.rate_multiplier < 1f32 {
                                self.rate_multiplier = 1f32;
                            }
                            self.status_text = Some(("Oh no!\nSlow down!", 120));
                            self.music.slow_down();
                            self.damaged_frames = DAMAGED_FRAMES_MAX;
                        }
                        self.music.damaged();
                    }
                }
                self.music.start();
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

        if let Some((x, building_type, state)) = &self.building {
            match building_type {
                Building::WrongHouse => crate::blit(
                    &HOUSE0,
                    round_f32_to_i32(*x),
                    30,
                    HOUSE0_WIDTH,
                    HOUSE0_HEIGHT,
                    HOUSE0_FLAGS,
                ),
                Building::House => match state {
                    0 => crate::blit(
                        &HOUSE1,
                        round_f32_to_i32(*x),
                        30 + HOUSE0_HEIGHT as i32 - HOUSE1_HEIGHT as i32,
                        HOUSE1_WIDTH,
                        HOUSE1_HEIGHT,
                        HOUSE1_FLAGS,
                    ),
                    1 => crate::blit(
                        &HOUSE0,
                        round_f32_to_i32(*x),
                        30,
                        HOUSE0_WIDTH,
                        HOUSE0_HEIGHT,
                        HOUSE0_FLAGS,
                    ),
                    _ => (),
                },
                Building::SpeedUp => crate::blit(
                    &SPEEDUP,
                    round_f32_to_i32(*x),
                    50,
                    SPEEDUP_WIDTH,
                    SPEEDUP_HEIGHT,
                    SPEEDUP_FLAGS,
                ),
                Building::SlowDown => crate::blit(
                    &SLOWDOWN,
                    round_f32_to_i32(*x),
                    50,
                    SLOWDOWN_WIDTH,
                    SLOWDOWN_HEIGHT,
                    SLOWDOWN_FLAGS,
                ),
            }
        }

        if self.damaged_frames % 4 < 2 || self.lives == 0 {
            if self.car_state {
                crate::blit(&CAR0, 10, 103, CAR0_WIDTH, CAR0_HEIGHT, CAR0_FLAGS);
            } else {
                crate::blit(&CAR1, 10, 103, CAR1_WIDTH, CAR1_HEIGHT, CAR1_FLAGS);
            }
        }

        if self.is_in_range {
            unsafe {
                *crate::DRAW_COLORS = 0x1;
            }
            if let Some((_, Building::WrongHouse, _)) = self.building {
                crate::text("Don't Press!", 5, 10);
            } else if let Some((_, Building::SpeedUp, _)) = self.building {
                crate::text(b"Press \x80?", 5, 10);
            } else if let Some((_, Building::SlowDown, _)) = self.building {
                crate::text(b"Press \x80?", 5, 10);
            } else {
                crate::text(b"Press \x80!", 5, 10);
            }
        }

        unsafe {
            *crate::DRAW_COLORS = 0x1;
        }

        match self.lives {
            3 => crate::text("3 HP", 0, 0),
            2 => crate::text("2 HP", 0, 0),
            1 => crate::text("1 HP", 0, 0),
            0 => crate::text("0 HP", 0, 0),
            _ => unreachable!(),
        }

        if let Some((s, t)) = self.status_text {
            if (t / 10) % 4 <= 1 {
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
                self.score_buf[width - 1 - i] = '0' as u8 + (temp % 10) as u8;
                temp /= 10;
            }
            for i in width..16 {
                self.score_buf[i] = 0;
            }
            if self.lives > 0 {
                crate::custom_text(self.score_buf, width, 160 - width as i32 * 8, 0);
            } else {
                crate::text("GAME OVER", 40, 40);
                crate::text("Score:", 50, 50);
                crate::custom_text(self.score_buf, width, 70 - (width / 2) as i32 * 8, 60);
                crate::text(b"Press \x81 to restart", 10, 70);
            }
        } else {
            if self.lives > 0 {
                crate::text("99999999999999", 160 - 10 * 8, 0);
            } else {
                crate::text("GAME OVER", 40, 40);
                crate::text("Score:", 50, 50);
                crate::text("99999999999999", 70 - 7 * 8, 60);
                crate::text(b"Press \x81 to restart", 10, 70);
            }
        }
    }
}
