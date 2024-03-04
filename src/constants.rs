pub const X_PIXELS: u32 = 64;
pub const Y_PIXELS: u32 = 32;
pub const PIXEL_SIZE: u32 = 16;
pub const PROGRAM_START: usize = 0x200;
pub const MEM_SIZE: usize = 4096;
pub const VRAM_SIZE: usize = (X_PIXELS * Y_PIXELS) as usize * 3;
pub const ON: u8 = 255;
pub const OFF: u8 = 0;
