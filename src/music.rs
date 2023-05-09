use crate::wasm4::*;

const FRAMES_PER_SIXTEENTH: f32 = 6.0f32;

const SLOWDOWN_RATE: f32 = 0.001f32;
const SLOWDOWN_REVERT_RATE: f32 = 0.002f32;
const SLOWDOWN_MIN: f32 = 0.6f32;

const SPEEDUP_RATE: f32 = 1.00f32;
const SPEEDUP_REVERT_RATE: f32 = 0.1f32;
const SPEEDUP_MAX: f32 = 1.6f32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Pitch {
    C0,
    D0,
    E0,
    F0,
    G0,
    A0,
    B0,
    C1,
    D1,
    E1,
    F1,
    G1,
    A1,
    B1,
    C2,
    D2,
    E2,
    F2,
    G2,
    A2,
    B2,
    C3,
    D3,
    E3,
    F3,
    G3,
    A3,
    B3,
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
    C5,
    D5,
    E5,
    F5,
    G5,
    A5,
    B5,
    C6,
    D6,
    E6,
    F6,
    G6,
    A6,
    B6,
    C7,
    D7,
    E7,
    F7,
    G7,
    A7,
    B7,
    C8,
    D8,
    E8,
    F8,
    G8,
    A8,
    B8,
}

impl Into<f32> for Pitch {
    fn into(self) -> f32 {
        match self {
            Pitch::C0 => 16.35,
            Pitch::D0 => 17.32,
            Pitch::E0 => 20.6,
            Pitch::F0 => 21.83,
            Pitch::G0 => 24.5,
            Pitch::A0 => 27.5,
            Pitch::B0 => 30.87,
            Pitch::C1 => 32.70,
            Pitch::D1 => 36.71,
            Pitch::E1 => 41.2,
            Pitch::F1 => 43.65,
            Pitch::G1 => 49.0,
            Pitch::A1 => 55.0,
            Pitch::B1 => 61.74,
            Pitch::C2 => 65.41,
            Pitch::D2 => 73.42,
            Pitch::E2 => 82.41,
            Pitch::F2 => 87.31,
            Pitch::G2 => 98.0,
            Pitch::A2 => 110.0,
            Pitch::B2 => 123.47,
            Pitch::C3 => 130.81,
            Pitch::D3 => 146.83,
            Pitch::E3 => 164.81,
            Pitch::F3 => 174.61,
            Pitch::G3 => 196.0,
            Pitch::A3 => 220.0,
            Pitch::B3 => 246.94,
            Pitch::C4 => 261.63,
            Pitch::D4 => 293.66,
            Pitch::E4 => 329.63,
            Pitch::F4 => 349.23,
            Pitch::G4 => 392.0,
            Pitch::A4 => 440.0,
            Pitch::B4 => 493.88,
            Pitch::C5 => 523.25,
            Pitch::D5 => 587.33,
            Pitch::E5 => 659.25,
            Pitch::F5 => 698.46,
            Pitch::G5 => 783.99,
            Pitch::A5 => 880.0,
            Pitch::B5 => 987.77,
            Pitch::C6 => 1046.5,
            Pitch::D6 => 1174.66,
            Pitch::E6 => 1318.51,
            Pitch::F6 => 1396.91,
            Pitch::G6 => 1567.98,
            Pitch::A6 => 1760.00,
            Pitch::B6 => 1975.53,
            Pitch::C7 => 2093.0,
            Pitch::D7 => 2349.32,
            Pitch::E7 => 2637.02,
            Pitch::F7 => 2793.83,
            Pitch::G7 => 3135.96,
            Pitch::A7 => 3520.00,
            Pitch::B7 => 3951.07,
            Pitch::C8 => 4186.01,
            Pitch::D8 => 4698.63,
            Pitch::E8 => 5274.04,
            Pitch::F8 => 5587.65,
            Pitch::G8 => 6271.93,
            Pitch::A8 => 7040.0,
            Pitch::B8 => 7902.13,
        }
    }
}

impl Pitch {
    pub fn to_u32(self) -> u32 {
        (Into::<f32>::into(self) + 0.5f32) as u32
    }

    pub fn to_u32_mult(self, mult: f32) -> u32 {
        (Into::<f32>::into(self) * mult + 0.5f32) as u32
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Duration {
    SIXTEENTH,
    EIGHTH,
    QUARTER,
    HALF,
    FULL,
}

impl Into<f32> for Duration {
    fn into(self) -> f32 {
        match self {
            Duration::SIXTEENTH => 1f32,
            Duration::EIGHTH => 2f32,
            Duration::QUARTER => 4f32,
            Duration::HALF => 8f32,
            Duration::FULL => 16f32,
        }
    }
}

const PULSE1_NOTES: [(Pitch, Duration, u8); 63] = [
    // m1
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::E4, Duration::EIGHTH, 40),
    (Pitch::D4, Duration::EIGHTH, 40),
    (Pitch::F4, Duration::EIGHTH, 40),
    (Pitch::E4, Duration::EIGHTH, 40),
    (Pitch::C6, Duration::EIGHTH, 40),
    (Pitch::B5, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    // m2
    (Pitch::D5, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    (Pitch::A5, Duration::EIGHTH, 40),
    (Pitch::A4, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::QUARTER, 40),
    // m3
    (Pitch::A4, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::F4, Duration::EIGHTH, 40),
    (Pitch::E4, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::F4, Duration::EIGHTH, 40),
    (Pitch::E4, Duration::EIGHTH, 40),
    (Pitch::D4, Duration::EIGHTH, 40),
    // m4
    (Pitch::C4, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::D5, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    (Pitch::C4, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    (Pitch::C6, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    // m5
    (Pitch::A4, Duration::EIGHTH, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::E5, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::A4, Duration::EIGHTH, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::D5, Duration::EIGHTH, 40),
    // m6
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::A4, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::B5, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    // m7
    (Pitch::A5, Duration::EIGHTH, 40),
    (Pitch::G5, Duration::EIGHTH, 40),
    (Pitch::F5, Duration::EIGHTH, 40),
    (Pitch::E5, Duration::EIGHTH, 40),
    (Pitch::D5, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::A4, Duration::EIGHTH, 40),
    // m8
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::F4, Duration::EIGHTH, 40),
    (Pitch::E4, Duration::EIGHTH, 40),
    (Pitch::D4, Duration::EIGHTH, 40),
    (Pitch::C4, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::C4, Duration::EIGHTH, 40),
];

const TRI_NOTES: [(Pitch, Duration, u8); 37] = [
    // m1
    (Pitch::E4, Duration::QUARTER, 40),
    (Pitch::G4, Duration::QUARTER, 40),
    (Pitch::C4, Duration::QUARTER, 40),
    (Pitch::G4, Duration::QUARTER, 40),
    // m2
    (Pitch::F5, Duration::QUARTER, 40),
    (Pitch::A4, Duration::QUARTER, 40),
    (Pitch::A4, Duration::QUARTER, 40),
    (Pitch::G5, Duration::QUARTER, 40),
    // m3
    (Pitch::F5, Duration::QUARTER, 40),
    (Pitch::G5, Duration::QUARTER, 40),
    (Pitch::C5, Duration::QUARTER, 40),
    (Pitch::G5, Duration::QUARTER, 40),
    // m4
    (Pitch::B4, Duration::QUARTER, 40),
    (Pitch::G5, Duration::QUARTER, 40),
    (Pitch::C4, Duration::EIGHTH, 40),
    (Pitch::G4, Duration::EIGHTH, 40),
    (Pitch::C4, Duration::QUARTER, 40),
    // m5
    (Pitch::A4, Duration::QUARTER, 40),
    (Pitch::A4, Duration::QUARTER, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::A4, Duration::EIGHTH, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::E5, Duration::EIGHTH, 40),
    // m6
    (Pitch::C5, Duration::QUARTER, 40),
    (Pitch::E5, Duration::QUARTER, 40),
    (Pitch::D5, Duration::QUARTER, 40),
    (Pitch::C5, Duration::QUARTER, 40),
    // m7
    (Pitch::A5, Duration::QUARTER, 40),
    (Pitch::E5, Duration::QUARTER, 40),
    (Pitch::A5, Duration::QUARTER, 40),
    (Pitch::E5, Duration::QUARTER, 40),
    // m8
    (Pitch::G5, Duration::QUARTER, 40),
    (Pitch::D5, Duration::QUARTER, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::B4, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
    (Pitch::C5, Duration::EIGHTH, 40),
];

pub struct Music {
    started: bool,
    pulse1_time: f32,
    pulse1_idx: usize,
    tri_time: f32,
    tri_idx: usize,
    factor: f32,
    factor_slowing: Option<bool>,
    factor_speeding: Option<bool>,
    game_over: bool,
}

impl Music {
    pub fn new() -> Self {
        Self {
            started: false,
            pulse1_time: 0f32,
            pulse1_idx: 0,
            tri_time: 0f32,
            tri_idx: 0,
            factor: 1f32,
            factor_slowing: None,
            factor_speeding: None,
            game_over: false,
        }
    }

    pub fn reset(&mut self) {
        self.started = false;
        self.pulse1_time = 0f32;
        self.pulse1_idx = 0;
        self.tri_time = 0f32;
        self.tri_idx = 0;
        self.factor = 1f32;
        self.factor_slowing = None;
        self.factor_speeding = None;
        self.game_over = false;
    }

    pub fn gameover(&mut self) {
        self.game_over = true;
    }

    pub fn speed_up(&mut self) {
        self.factor_slowing = None;
        //self.factor_speeding = Some(true);
        self.factor_speeding = None;
        self.factor = 1f32;
        crate::tone(
            Pitch::C2.to_u32() | (Pitch::C7.to_u32() << 16),
            30 | (30 << 8),
            40,
            TONE_PULSE2,
        );
    }

    pub fn slow_down(&mut self) {
        self.factor_slowing = Some(true);
        self.factor_speeding = None;
        self.factor = 1f32;
    }

    pub fn damaged(&self) {
        if self.started {
            crate::tone(
                Pitch::D5.to_u32() | (Pitch::D1.to_u32() << 16),
                30 << 8,
                60,
                TONE_NOISE,
            )
        }
    }

    fn get_factor(&self) -> f32 {
        self.factor
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn update(&mut self) {
        if !self.started {
            return;
        }

        if self.game_over {
            self.factor -= SLOWDOWN_RATE;
            if self.factor <= 0f32 {
                self.reset();
            }
        } else {
            if let Some(not_reverting) = &mut self.factor_slowing {
                if *not_reverting {
                    self.factor -= SLOWDOWN_RATE;
                    if self.factor <= SLOWDOWN_MIN {
                        *not_reverting = false;
                        self.factor = SLOWDOWN_MIN;
                    }
                } else {
                    self.factor += SLOWDOWN_REVERT_RATE;
                    if self.factor >= 1f32 {
                        self.factor = 1f32;
                        self.factor_slowing.take();
                    }
                }
            } else if let Some(not_reverting) = &mut self.factor_speeding {
                if *not_reverting {
                    self.factor += SPEEDUP_RATE;
                    if self.factor >= SPEEDUP_MAX {
                        *not_reverting = false;
                        self.factor = SPEEDUP_MAX;
                    }
                } else {
                    self.factor -= SPEEDUP_REVERT_RATE;
                    if self.factor <= 1f32 {
                        self.factor = 1f32;
                        self.factor_speeding.take();
                    }
                }
            }
        }

        if self.pulse1_idx < PULSE1_NOTES.len() {
            if self.pulse1_time <= 0f32 {
                let frames =
                    Into::<f32>::into(PULSE1_NOTES[self.pulse1_idx].1) * FRAMES_PER_SIXTEENTH;
                //                    / self.get_factor();
                crate::tone(
                    PULSE1_NOTES[self.pulse1_idx]
                        .0
                        .to_u32_mult(self.get_factor()),
                    ((frames + 0.5f32) as u32) << 8,
                    PULSE1_NOTES[self.pulse1_idx].2 as u32,
                    TONE_PULSE1,
                );
                self.pulse1_time += frames;
                self.pulse1_idx += 1;
            }
        }
        self.pulse1_time -= self.get_factor();
        //self.pulse1_time -= 1f32;

        if self.tri_idx < TRI_NOTES.len() {
            if self.tri_time <= 0f32 {
                let frames = Into::<f32>::into(TRI_NOTES[self.tri_idx].1) * FRAMES_PER_SIXTEENTH;
                //                    / self.get_factor();
                crate::tone(
                    TRI_NOTES[self.tri_idx].0.to_u32_mult(self.get_factor()),
                    ((frames + 0.5f32) as u32) << 8,
                    TRI_NOTES[self.tri_idx].2 as u32,
                    TONE_TRIANGLE,
                );
                self.tri_time += frames;
                self.tri_idx += 1;
            }
        }
        self.tri_time -= self.get_factor();
        //self.tri_time -= 1f32;

        if self.pulse1_idx >= PULSE1_NOTES.len()
            && self.tri_idx >= TRI_NOTES.len()
            && self.pulse1_time <= 0f32
            && self.tri_time <= 0f32
        {
            self.pulse1_idx = 0;
            self.tri_idx = 0;
        }
    }
}
