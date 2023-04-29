use crate::wasm4::*;

const C4: u32 = 262;
const D4: u32 = 293;
const E4: u32 = 330;
const F4: u32 = 349;
const G4: u32 = 392;
const A4: u32 = 440;
const B4: u32 = 494;
const C5: u32 = 523;
const D5: u32 = 587;
const E5: u32 = 659;
const F5: u32 = 698;
const G5: u32 = 784;
const A5: u32 = 880;
const B5: u32 = 988;
const C6: u32 = 1047;

pub struct Music {
    started: bool,
    frame: u32,
}

impl Music {
    pub fn new() -> Self {
        Self {
            started: false,
            frame: 0,
        }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn update(&mut self) {
        if !self.started {
            return;
        }

        // m1
        if self.frame == 0 {
            tone(G4, 10 << 8, 40, TONE_PULSE1);
            tone(E4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 10 {
            tone(E4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 20 {
            tone(D4, 10 << 8, 40, TONE_PULSE1);
            tone(G4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 30 {
            tone(F4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 40 {
            tone(E4, 10 << 8, 40, TONE_PULSE1);
            tone(C4, 30 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 50 {
            tone(C6, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 60 {
            tone(B5, 10 << 8, 40, TONE_PULSE1);
            tone(G4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 70 {
            tone(G5, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 80 {
            tone(D5, 10 << 8, 40, TONE_PULSE1);
            tone(F5, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 90 {
            tone(G5, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 100 {
            tone(A5, 10 << 8, 40, TONE_PULSE1);
            tone(A4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 110 {
            tone(A4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 120 {
            tone(G5, 10 << 8, 40, TONE_PULSE1);
            tone(A4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 130 {
            tone(G4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 140 {
            tone(G4, 20 << 8, 40, TONE_PULSE1);
            tone(G5, 20 << 8, 40, TONE_TRIANGLE);
        }
        // m3
        else if self.frame == 160 {
            tone(A4, 10 << 8, 40, TONE_PULSE1);
            tone(F5, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 170 {
            tone(G4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 180 {
            tone(F4, 10 << 8, 40, TONE_PULSE1);
            tone(G5, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 190 {
            tone(E4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 200 {
            tone(G4, 10 << 8, 40, TONE_PULSE1);
            tone(C5, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 210 {
            tone(F4, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 220 {
            tone(E4, 10 << 8, 40, TONE_PULSE1);
            tone(G5, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 230 {
            tone(D4, 10 << 8, 40, TONE_PULSE1);
        }
        // m4
        else if self.frame == 240 {
            tone(C4, 10 << 8, 40, TONE_PULSE1);
            tone(B4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 250 {
            tone(C5, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 260 {
            tone(D5, 10 << 8, 40, TONE_PULSE1);
            tone(G5, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 270 {
            tone(G5, 10 << 8, 40, TONE_PULSE1);
        } else if self.frame == 280 {
            tone(C4, 10 << 8, 40, TONE_PULSE1);
            tone(C4, 10 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 290 {
            tone(G5, 10 << 8, 40, TONE_PULSE1);
            tone(G4, 10 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 300 {
            tone(C6, 10 << 8, 40, TONE_PULSE1);
            tone(C4, 20 << 8, 40, TONE_TRIANGLE);
        } else if self.frame == 310 {
            tone(G5, 10 << 8, 40, TONE_PULSE1);
        }
        self.frame += 1;
        if self.frame == 320 {
            self.frame = 0;
        }
    }
}
