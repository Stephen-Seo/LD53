// car0
pub const CAR0_WIDTH: u32 = 48;
pub const CAR0_HEIGHT: u32 = 32;
pub const CAR0_FLAGS: u32 = 1; // BLIT_2BPP
pub const CAR0: [u8; 384] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x3f, 0xff, 0xff, 0xfc, 0xa8, 0x0a, 0xaa, 0xaa, 0x00, 0x00, 0xaa, 0xaa, 0x1f, 0xff, 0xff,
    0xf0, 0x00, 0x0a, 0xaa, 0xaa, 0x80, 0x02, 0xaa, 0xaa, 0x1f, 0xff, 0xff, 0xf2, 0xa8, 0x0a, 0xaa,
    0xaa, 0x80, 0x02, 0xaa, 0xaa, 0x17, 0xff, 0xff, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x15, 0xff, 0xff, 0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0xff, 0xff,
    0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x7f, 0xff, 0xca, 0xa8, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x5f, 0xff, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x15, 0x5f, 0xff, 0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x57, 0xff,
    0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x55, 0xff, 0xca, 0xa8, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x55, 0xff, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x05, 0x55, 0x7f, 0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x05, 0x55, 0x5f,
    0xc0, 0x00, 0x0a, 0xaa, 0xa0, 0x20, 0x08, 0x0a, 0xaa, 0x00, 0x15, 0x5f, 0xca, 0xa8, 0x0a, 0xaa,
    0xaa, 0x20, 0x08, 0xaa, 0xaa, 0x00, 0x00, 0x03, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x00, 0xaa, 0xa3, 0xca, 0xa0, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0x00, 0x03,
    0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0xaa, 0xa3, 0xca, 0xa0, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0x00, 0x03, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x00, 0x2a, 0xa3, 0xca, 0x80, 0x00, 0x0a, 0xaa, 0xa0, 0x0a, 0xaa, 0x00, 0x00, 0x00, 0x03,
    0xc0, 0x00, 0xaa, 0x82, 0xaa, 0xa0, 0x0a, 0xa8, 0x2a, 0xa0, 0x2a, 0xa3, 0xca, 0x02, 0xff, 0xe0,
    0xaa, 0xa0, 0x0a, 0xa0, 0xbf, 0xf8, 0x00, 0x03, 0xc0, 0x0b, 0x00, 0x38, 0x00, 0x00, 0x00, 0x02,
    0xc0, 0x0e, 0x0a, 0xa3, 0xc0, 0x2c, 0x40, 0x4e, 0x00, 0x00, 0x00, 0x0b, 0x10, 0x13, 0x80, 0x03,
    0xc0, 0xb1, 0x00, 0x13, 0x80, 0x00, 0x00, 0x2c, 0x40, 0x04, 0xe2, 0xa3, 0xc0, 0xb1, 0x00, 0x13,
    0x80, 0x00, 0x00, 0x2c, 0x40, 0x04, 0xe0, 0x03, 0xff, 0xfc, 0x40, 0x4f, 0xff, 0xff, 0xff, 0xff,
    0x10, 0x13, 0xff, 0xff, 0xff, 0xff, 0x00, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xc0, 0x0f, 0xff, 0xff,
];
// car1
pub const CAR1_WIDTH: u32 = 48;
pub const CAR1_HEIGHT: u32 = 32;
pub const CAR1_FLAGS: u32 = 1; // BLIT_2BPP
pub const CAR1: [u8; 384] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xff, 0xff, 0xfc, 0xa8, 0x0a, 0xaa, 0xaa, 0x00, 0x00, 0xaa,
    0xaa, 0x1f, 0xff, 0xff, 0xf0, 0x00, 0x0a, 0xaa, 0xaa, 0x80, 0x02, 0xaa, 0xaa, 0x1f, 0xff, 0xff,
    0xf2, 0xa8, 0x0a, 0xaa, 0xaa, 0x80, 0x02, 0xaa, 0xaa, 0x17, 0xff, 0xff, 0xc0, 0x00, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0xff, 0xff, 0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x15, 0xff, 0xff, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x7f, 0xff,
    0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x5f, 0xff, 0xc0, 0x00, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x5f, 0xff, 0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x15, 0x57, 0xff, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x55, 0xff,
    0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x15, 0x55, 0xff, 0xc0, 0x00, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x05, 0x55, 0x7f, 0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x05, 0x55, 0x5f, 0xc0, 0x00, 0x0a, 0xaa, 0xa0, 0x20, 0x08, 0x0a, 0xaa, 0x00, 0x15, 0x5f,
    0xca, 0xa8, 0x0a, 0xaa, 0xaa, 0x20, 0x08, 0xaa, 0xaa, 0x00, 0x00, 0x03, 0xc0, 0x00, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0xaa, 0xa3, 0xca, 0xa0, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa,
    0xaa, 0x00, 0x00, 0x03, 0xc0, 0x00, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0xaa, 0xa3,
    0xca, 0xa0, 0x0a, 0xaa, 0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0x00, 0x03, 0xc0, 0x00, 0x0a, 0xaa,
    0xaa, 0xa0, 0x0a, 0xaa, 0xaa, 0x00, 0x2a, 0xa3, 0xca, 0x80, 0x00, 0x0a, 0xaa, 0xa0, 0x0a, 0xaa,
    0x00, 0x00, 0x00, 0x03, 0xc0, 0x00, 0xaa, 0x82, 0xaa, 0xa0, 0x0a, 0xa8, 0x2a, 0xa0, 0x2a, 0xa3,
    0xca, 0x02, 0xff, 0xe0, 0xaa, 0xa0, 0x0a, 0xa0, 0xbf, 0xf8, 0x00, 0x03, 0xc0, 0x0b, 0xff, 0xf8,
    0x00, 0x00, 0x00, 0x02, 0xff, 0xfe, 0x0a, 0xa3, 0xc0, 0x2f, 0x04, 0x3e, 0x00, 0x00, 0x00, 0x0b,
    0xc1, 0x0f, 0x80, 0x03, 0xc0, 0xbc, 0x51, 0x4f, 0x80, 0x00, 0x00, 0x2f, 0x14, 0x53, 0xe2, 0xa3,
    0xc0, 0xb0, 0x00, 0x03, 0x80, 0x00, 0x00, 0x2c, 0x00, 0x00, 0xe0, 0x03, 0xff, 0xf0, 0x00, 0x03,
    0xff, 0xff, 0xff, 0xfc, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfc, 0x51, 0x4f, 0xff, 0xff, 0xff, 0xff,
    0x14, 0x53, 0xff, 0xff, 0xff, 0xff, 0x04, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xc1, 0x0f, 0xff, 0xff,
];
