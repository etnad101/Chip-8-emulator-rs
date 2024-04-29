use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputManager {
    pub keys: u16,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager { keys: 0 }
    }

    pub fn check_key_pressed(&self, key: u16) -> bool {
        todo!("check_key_down");
    }

    pub fn check_key_released(&self, key: u16) -> bool {
        todo!("check_key_released");
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
        match event {
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::X) => self.keys |= 1 << 0x0,
                Some(Keycode::Num1) => self.keys |= 1 << 0x1,
                Some(Keycode::Num2) => self.keys |= 1 << 0x2,
                Some(Keycode::Num3) => self.keys |= 1 << 0x3,
                Some(Keycode::Q) => self.keys |= 1 << 0x4,
                Some(Keycode::W) => self.keys |= 1 << 0x5,
                Some(Keycode::E) => self.keys |= 1 << 0x6,
                Some(Keycode::A) => self.keys |= 1 << 0x7,
                Some(Keycode::S) => self.keys |= 1 << 0x8,
                Some(Keycode::D) => self.keys |= 1 << 0x9,
                Some(Keycode::Z) => self.keys |= 1 << 0xA,
                Some(Keycode::C) => self.keys |= 1 << 0xB,
                Some(Keycode::Num4) => self.keys |= 1 << 0xC,
                Some(Keycode::R) => self.keys |= 1 << 0xD,
                Some(Keycode::F) => self.keys |= 1 << 0xE,
                Some(Keycode::V) => self.keys |= 1 << 0xF,
                _ => (),
            },

            Event::KeyUp { keycode, .. } => match keycode {
                Some(Keycode::X) => self.keys &= 0b1111_1111_1111_1110,
                Some(Keycode::Num1) => self.keys &= 0b1111_1111_1111_1101,
                Some(Keycode::Num2) => self.keys &= 0b1111_1111_1111_1011,
                Some(Keycode::Num3) => self.keys &= 0b1111_1111_1111_0111,
                Some(Keycode::Q) => self.keys &= 0b1111_1111_1110_1111,
                Some(Keycode::W) => self.keys &= 0b1111_1111_1101_1111,
                Some(Keycode::E) => self.keys &= 0b1111_1111_1011_1111,
                Some(Keycode::A) => self.keys &= 0b1111_1111_0111_1111,
                Some(Keycode::S) => self.keys &= 0b1111_1110_1111_1111,
                Some(Keycode::D) => self.keys &= 0b1111_1101_1111_1111,
                Some(Keycode::Z) => self.keys &= 0b1111_1011_1111_1111,
                Some(Keycode::C) => self.keys &= 0b1111_0111_1111_1111,
                Some(Keycode::Num4) => self.keys &= 0b1110_1111_1111_1111,
                Some(Keycode::R) => self.keys &= 0b1101_1111_1111_1111,
                Some(Keycode::F) => self.keys &= 0b1011_1111_1111_1111,
                Some(Keycode::V) => self.keys &= 0b0111_1111_1111_1111,
                _ => (),
            },

            _ => (),
        }
    }
}
