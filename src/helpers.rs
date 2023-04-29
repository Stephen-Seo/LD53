pub fn fill(color: u8) {
    if color > 3 {
        crate::trace("ERROR: Invalid value passed to helper \"fill\"");
        return;
    }
    unsafe {
        (&mut *crate::FRAMEBUFFER).fill(color | (color << 2) | (color << 4) | (color << 6));
    }
}

pub fn round_f32_to_i32(f: f32) -> i32 {
    (f + 0.5f32) as i32
}
