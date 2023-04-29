use crate::sprites::*;
use tinyrand::{Rand, StdRand};

const CAR_ANIM_FRAMES: u8 = 10;
const CAMERA_RATE: f32 = 0.07f32;
const BOX_INTERP_RATE: f32 = 0.01f32;
const HOUSE_MAX_DIST: f32 = 10.0f32;

pub struct World {
    // x, y
    shrubs: [(i32, i32); 4],
    // x, y, state
    // state:
    //   - 0 does not exist
    //   - 1 exists
    //   - 2 destroyed
    houses: [(i32, i32, u8); 2],
    // u8:
    //   - 0 facing right
    //   - 1 facing left
    //   - 2 facing up
    //   - 3 facing down
    //   - 4 facing right alt
    //   - 5 facing left alt
    //   - 6 facing up alt
    //   - 7 facing down alt
    car_pos: (i32, i32, u8),
    car_frames: u8,
    camera_pos: (f32, f32),
    // x, y, interp, house_idx
    box_pos: Option<(f32, f32, f32, usize)>,
    rand: StdRand,
}

impl World {
    pub fn new() -> World {
        World {
            shrubs: [(-40, 0), (0, 40), (40, 0), (0, -40)],
            houses: [(300, 0, 2), (0, 0, 0)],
            car_pos: (0, 0, 0),
            car_frames: 0,
            camera_pos: (-80f32, -80f32),
            box_pos: None,
            rand: StdRand::default(),
        }
    }

    fn car_to_left(&mut self) {
        if self.car_pos.2 > 3 {
            self.car_pos.2 = 5;
        } else {
            self.car_pos.2 = 1;
        }
    }

    fn car_to_right(&mut self) {
        if self.car_pos.2 > 3 {
            self.car_pos.2 = 4;
        } else {
            self.car_pos.2 = 0;
        }
    }

    fn car_to_up(&mut self) {
        if self.car_pos.2 > 3 {
            self.car_pos.2 = 6;
        } else {
            self.car_pos.2 = 2;
        }
    }

    fn car_to_down(&mut self) {
        if self.car_pos.2 > 3 {
            self.car_pos.2 = 7;
        } else {
            self.car_pos.2 = 3;
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *crate::GAMEPAD1 };

        if gamepad & crate::BUTTON_UP != 0 {
            self.car_pos.1 -= 1;
            self.car_to_up();
            self.rand.next_u16();
        }
        if gamepad & crate::BUTTON_DOWN != 0 {
            self.car_pos.1 += 1;
            self.car_to_down();
            self.rand.next_u16();
        }
        if gamepad & crate::BUTTON_LEFT != 0 {
            self.car_pos.0 -= 1;
            self.car_to_left();
            self.rand.next_u16();
        }
        if gamepad & crate::BUTTON_RIGHT != 0 {
            self.car_pos.0 += 1;
            self.car_to_right();
            self.rand.next_u16();
        }

        self.camera_pos.0 += ((self.car_pos.0 as f32 - 80f32) - self.camera_pos.0) * CAMERA_RATE;
        self.camera_pos.1 += ((self.car_pos.1 as f32 - 80f32) - self.camera_pos.1) * CAMERA_RATE;

        let mut house_repaired = false;
        for (x, y, state) in &mut self.houses {
            if *state == 2 {
                let dx = self.car_pos.0 - *x;
                let dy = self.car_pos.1 - *y;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= HOUSE_MAX_DIST {
                    *state = 1;
                    house_repaired = true;
                }
            }
        }
        if house_repaired {
            for (x, y, state) in &mut self.houses {
                if *state == 0 {
                    *state = 2;
                    // TODO
                    match self.rand.next_u16() % 4 {
                        0 => (),
                        1 => (),
                        2 => (),
                        3 => (),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            *crate::DRAW_COLORS = 0x312;
        }

        for (x, y) in self.shrubs {
            crate::blit(
                &PLANT,
                ((x - PLANT_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round() as i32,
                ((y - PLANT_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round() as i32,
                PLANT_WIDTH,
                PLANT_HEIGHT,
                PLANT_FLAGS,
            )
        }

        for (x, y, flags) in &mut self.houses {
            //            if self.car_frames == 0 {
            //                if *flags == 1 {
            //                    *flags = 2;
            //                } else if *flags == 2 {
            //                    *flags = 1;
            //                }
            //            }
            match flags {
                0 => (),
                1 => crate::blit(
                    &HOUSE0,
                    ((*x - HOUSE0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round() as i32,
                    ((*y - HOUSE0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round() as i32,
                    HOUSE0_WIDTH,
                    HOUSE0_HEIGHT,
                    HOUSE0_FLAGS,
                ),
                2 => crate::blit(
                    &HOUSE1,
                    ((*x - HOUSE1_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round() as i32,
                    ((*y - HOUSE0_HEIGHT as i32 / 2 + (HOUSE0_HEIGHT - HOUSE1_HEIGHT) as i32)
                        as f32
                        - self.camera_pos.1)
                        .round() as i32,
                    HOUSE1_WIDTH,
                    HOUSE1_HEIGHT,
                    HOUSE1_FLAGS,
                ),
                _ => (),
            }
        }

        self.car_frames += 1;
        if self.car_frames > CAR_ANIM_FRAMES {
            self.car_frames = 0;
            if self.car_pos.2 > 3 {
                self.car_pos.2 -= 4;
            } else {
                self.car_pos.2 += 4;
            }
        }

        match self.car_pos.2 {
            0 => crate::blit(
                &CAR0,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR0_WIDTH,
                CAR0_HEIGHT,
                CAR0_FLAGS,
            ),
            1 => crate::blit(
                &CAR0,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR0_WIDTH,
                CAR0_HEIGHT,
                CAR0_FLAGS | crate::BLIT_FLIP_X,
            ),
            2 => crate::blit(
                &CAR0,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR0_WIDTH,
                CAR0_HEIGHT,
                CAR0_FLAGS | crate::BLIT_ROTATE,
            ),
            3 => crate::blit(
                &CAR0,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR0_WIDTH,
                CAR0_HEIGHT,
                CAR0_FLAGS | crate::BLIT_ROTATE | crate::BLIT_FLIP_X,
            ),
            4 => crate::blit(
                &CAR1,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR1_WIDTH,
                CAR1_HEIGHT,
                CAR1_FLAGS,
            ),
            5 => crate::blit(
                &CAR1,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR1_WIDTH,
                CAR1_HEIGHT,
                CAR1_FLAGS | crate::BLIT_FLIP_X,
            ),
            6 => crate::blit(
                &CAR1,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR1_WIDTH,
                CAR1_HEIGHT,
                CAR1_FLAGS | crate::BLIT_ROTATE,
            ),
            7 => crate::blit(
                &CAR1,
                ((self.car_pos.0 - CAR0_WIDTH as i32 / 2) as f32 - self.camera_pos.0).round()
                    as i32,
                ((self.car_pos.1 - CAR0_HEIGHT as i32 / 2) as f32 - self.camera_pos.1).round()
                    as i32,
                CAR1_WIDTH,
                CAR1_HEIGHT,
                CAR1_FLAGS | crate::BLIT_ROTATE | crate::BLIT_FLIP_X,
            ),
            _ => (),
        }
    }
}
