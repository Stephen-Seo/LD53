pub fn fill(color: u8) {
    if color > 3 {
        crate::trace("ERROR: Invalid value passed to helper \"fill\"");
        return;
    }
    unsafe {
        (&mut *crate::FRAMEBUFFER).fill(color | (color << 2) | (color << 4) | (color << 6));
    }
}
