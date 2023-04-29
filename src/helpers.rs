static mut CAR_STATE: u8 = 0;
static mut CAR_STATE_FRAMES: u8 = 0;
const CAR_FRAMES: u8 = 10;

pub fn fill(color: u8) {
    if color > 3 {
        crate::trace("ERROR: Invalid value passed to helper \"fill\"");
        return;
    }
    unsafe {
        (&mut *crate::FRAMEBUFFER).fill(color | (color << 2) | (color << 4) | (color << 6));
    }
}

pub fn draw_mouse() -> Option<(i16, i16)> {
    let mouse = unsafe { *crate::MOUSE_BUTTONS };
    let mouse_x = unsafe { *crate::MOUSE_X };
    let mouse_y = unsafe { *crate::MOUSE_Y };
    if mouse & crate::MOUSE_LEFT != 0 {
        unsafe { *crate::DRAW_COLORS = 2 }
        crate::rect(i32::from(mouse_x) - 4, i32::from(mouse_y) - 4, 8, 8);
        Some((mouse_x, mouse_y))
    } else {
        unsafe { *crate::DRAW_COLORS = 1 }
        crate::rect(i32::from(mouse_x) - 2, i32::from(mouse_y) - 2, 4, 4);
        None
    }
}

pub fn toggle_car_state() -> bool {
    unsafe {
        CAR_STATE_FRAMES += 1;
        if CAR_STATE_FRAMES >= CAR_FRAMES {
            CAR_STATE_FRAMES = 0;
            if CAR_STATE == 0 {
                CAR_STATE = 1;
            } else {
                CAR_STATE = 0;
            }
        }
        CAR_STATE != 0
    }
}
