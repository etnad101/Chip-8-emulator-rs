use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputManager {
    pub keys: u16,
    prev_keys: u16,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keys: 0,
            prev_keys: 0,
        }
    }

    pub fn check_key_pressed(&self, key: u8) -> bool {
        let key: u16 = 1 << key;
        self.keys & key > 0
    }

    pub fn check_key_released(&self, p_key: u8) -> bool {
        let key: u16 = 1 << p_key;
        if self.prev_keys & key > 0 {
            println!("key down");
            if self.keys & key == 0 {
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn get_key_pressed(&self) -> u8 {
        let mut key = self.keys;
        let mut i: u8 = 0;
        while key > 1 {
            key >>= 1;
            i += 1;
        }
        i
    }

    pub fn any_key_pressed(&self) -> bool {
        self.keys == 0
    }

    pub fn handle_keyboard_input(&mut self, event: Event) {
        self.prev_keys = self.keys;
        println!("{:016b}, {:016b}", self.keys, self.prev_keys);
        match event {
            Event::KeyDown { keycode, .. } => {
                let key = match keycode {
                    Some(Keycode::X) => 1 << 0x0,
                    Some(Keycode::Num1) => 1 << 0x1,
                    Some(Keycode::Num2) => 1 << 0x2,
                    Some(Keycode::Num3) => 1 << 0x3,
                    Some(Keycode::Q) => 1 << 0x4,
                    Some(Keycode::W) => 1 << 0x5,
                    Some(Keycode::E) => 1 << 0x6,
                    Some(Keycode::A) => 1 << 0x7,
                    Some(Keycode::S) => 1 << 0x8,
                    Some(Keycode::D) => 1 << 0x9,
                    Some(Keycode::Z) => 1 << 0xA,
                    Some(Keycode::C) => 1 << 0xB,
                    Some(Keycode::Num4) => 1 << 0xC,
                    Some(Keycode::R) => 1 << 0xD,
                    Some(Keycode::F) => 1 << 0xE,
                    Some(Keycode::V) => 1 << 0xF,
                    _ => 0,
                };
                self.keys |= key
            }

            Event::KeyUp { keycode, .. } => {
                let key = match keycode {
                    Some(Keycode::X) => 0b1111_1111_1111_1110,
                    Some(Keycode::Num1) => 0b1111_1111_1111_1101,
                    Some(Keycode::Num2) => 0b1111_1111_1111_1011,
                    Some(Keycode::Num3) => 0b1111_1111_1111_0111,
                    Some(Keycode::Q) => 0b1111_1111_1110_1111,
                    Some(Keycode::W) => 0b1111_1111_1101_1111,
                    Some(Keycode::E) => 0b1111_1111_1011_1111,
                    Some(Keycode::A) => 0b1111_1111_0111_1111,
                    Some(Keycode::S) => 0b1111_1110_1111_1111,
                    Some(Keycode::D) => 0b1111_1101_1111_1111,
                    Some(Keycode::Z) => 0b1111_1011_1111_1111,
                    Some(Keycode::C) => 0b1111_0111_1111_1111,
                    Some(Keycode::Num4) => 0b1110_1111_1111_1111,
                    Some(Keycode::R) => 0b1101_1111_1111_1111,
                    Some(Keycode::F) => 0b1011_1111_1111_1111,
                    Some(Keycode::V) => 0b0111_1111_1111_1111,
                    _ => u16::MAX,
                };
                self.keys &= key;
            }

            _ => (),
        }
    }
}
