use rand::Rng;

use crate::drivers::input_driver::InputManager;

use crate::{
    config::{Config, ConfigFlags},
    constants::*,
};

// Build in chip8 characters
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {
    pub memory: [u8; MEM_SIZE],
    pub pc: usize,
    pub reg_i: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub input: InputManager,
    pub reg_v: [u8; 16],
    pub vram: [u8; 64 * 32 * 3],
    pub update_screen: bool,
    pub config: Config,
}

impl CPU {
    pub fn new(config: Config) -> CPU {
        let mut cpu = CPU {
            memory: [0; MEM_SIZE],
            pc: PROGRAM_START,
            reg_i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            input: InputManager::new(),
            reg_v: [0; 16],
            vram: [0; 64 * 32 * 3],
            update_screen: true,
            config,
        };

        cpu.memory[FONT_ADDR..(FONT.len() + FONT_ADDR)].copy_from_slice(&FONT[..]);

        cpu
    }

    // Debug memory viwer
    pub fn dump_mem(&self) {
        let mut x = 0;
        for i in 0..32 {
            if i % 16 == 0 {
                print!("|memAddr| ");
                x = 0;
            }
            print!(" {:02x} ", x);
            x += 1;
        }
        for i in 0..MEM_SIZE {
            if i % 32 == 0 {
                println!();
            }
            if i % 16 == 0 {
                print!("| {:#05x} | ", i);
            }

            print!(" {:02x} ", self.memory[i]);
        }
        println!();
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[PROGRAM_START..(program.len() + PROGRAM_START)].copy_from_slice(&program[..])
    }

    // Main loop

    fn fetch(&mut self) -> (u8, u8, u8, u8) {
        let first_byte = self.memory[self.pc];
        let second_byte = self.memory[self.pc + 1];
        let instruction: u16 = (first_byte as u16) << 8 | second_byte as u16;
        self.pc += 2;

        (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        )
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            // Fetch
            let instruction = self.fetch();

            // Decode
            let x = instruction.1 as usize;
            let y = instruction.2 as usize;
            let n = instruction.3 as usize;
            let nn = y << 4 | n;
            let nnn = x << 8 | y << 4 | n;
            // Execute
            match instruction {
                (0x0, 0x0, 0xe, 0x0) => self.clear_screen(),
                (0x0, 0x0, 0xe, 0xe) => self.sub_return(),
                (0xf, _, 0x6, 0x5) => self.load_mem(x),
                (0xf, _, 0x5, 0x5) => self.store_mem(x),
                (0xf, _, 0x3, 0x3) => self.bcd_conversion(x),
                (0xf, _, 0x2, 0x9) => self.font_character(x),
                (0xf, _, 0x1, 0xe) => self.add_to_index(x),
                (0xf, _, 0x1, 0x8) => self.set_sound_timer(x),
                (0xf, _, 0x1, 0x5) => self.set_delay_timer(x),
                (0xf, _, 0x0, 0xa) => self.get_key(x),
                (0xf, _, 0x0, 0x7) => self.get_delay_timer(x),
                (0xe, _, 0xa, 0x1) => self.skip_if_up(x),
                (0xe, _, 0x9, 0xe) => self.skip_if_down(x),
                (0x9, _, _, 0x0) => self.vy_skip_not_eq(x, y),
                (0x8, _, _, 0x0) => self.set_vx(x, y),
                (0x8, _, _, 0x1) => self.binary_or(x, y),
                (0x8, _, _, 0x2) => self.binary_and(x, y),
                (0x8, _, _, 0x3) => self.logical_xor(x, y),
                (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
                (0x8, _, _, 0x5) => self.vx_sub_vy(x, y),
                (0x8, _, _, 0x6) => self.shift_right(x, y),
                (0x8, _, _, 0x7) => self.vy_sub_vx(x, y),
                (0x8, _, _, 0xe) => self.shift_left(x, y),
                (0x5, _, _, 0x0) => self.vy_skip_eq(x, y),
                (0xd, _, _, _) => self.display(x, y, n),
                (0xc, _, _, _) => self.random(x, nn),
                (0xb, _, _, _) => self.jump_offset(x, nnn),
                (0xa, _, _, _) => self.set_index(nnn),
                (0x7, _, _, _) => self.add_reg_v(x, nn),
                (0x6, _, _, _) => self.set_reg_v(x, nn),
                (0x4, _, _, _) => self.vx_skip_not_eq(x, nn),
                (0x3, _, _, _) => self.vx_skip_eq(x, nn),
                (0x2, _, _, _) => self.subroutine(nnn),
                (0x1, _, _, _) => self.jump(nnn),
                _ => todo!(
                    "Unimplemented opcode: {:#x}, {:#x}, {:#x}, {:#x} @ pc = {:#x}",
                    instruction.0,
                    instruction.1,
                    instruction.2,
                    instruction.3,
                    self.pc - 2
                ),
            }
        }
    }

    // Opcodes
    fn clear_screen(&mut self) {
        for i in 0..VRAM_SIZE {
            self.vram[i] = 0;
        }
        self.update_screen = true;
    }

    fn set_index(&mut self, nnn: usize) {
        self.reg_i = nnn as u16;
    }

    fn set_reg_v(&mut self, x: usize, nn: usize) {
        self.reg_v[x] = nn as u8;
    }

    fn add_reg_v(&mut self, x: usize, nn: usize) {
        let mut vx = self.reg_v[x];
        vx = vx.wrapping_add(nn as u8);
        self.reg_v[x] = vx;
    }

    fn jump(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    fn subroutine(&mut self, nnn: usize) {
        self.stack.push(self.pc as u16);
        self.pc = nnn;
    }

    fn sub_return(&mut self) {
        self.pc = self.stack.pop().unwrap() as usize;
    }

    fn vx_skip_eq(&mut self, x: usize, nn: usize) {
        let vx = self.reg_v[x] as usize;
        if vx == nn {
            self.pc += 2;
        }
    }

    fn vx_skip_not_eq(&mut self, x: usize, nn: usize) {
        let vx = self.reg_v[x] as usize;
        if vx != nn {
            self.pc += 2;
        }
    }

    fn vy_skip_eq(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x] as usize;
        let vy = self.reg_v[y] as usize;
        if vx == vy {
            self.pc += 2;
        }
    }

    fn vy_skip_not_eq(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x] as usize;
        let vy = self.reg_v[y] as usize;
        if vx != vy {
            self.pc += 2;
        }
    }

    fn set_vx(&mut self, x: usize, y: usize) {
        let vy = self.reg_v[y];
        self.reg_v[x] = vy;
    }

    fn binary_or(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x];
        let vy = self.reg_v[y];
        self.reg_v[0xF] = 0;
        self.reg_v[x] = vx | vy;
    }

    fn binary_and(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x];
        let vy = self.reg_v[y];
        self.reg_v[0xF] = 0;
        self.reg_v[x] = vx & vy;
    }

    fn logical_xor(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x];
        let vy = self.reg_v[y];
        self.reg_v[0xF] = 0;
        self.reg_v[x] = vx ^ vy;
    }

    fn add_vx_vy(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x];
        let vy = self.reg_v[y];

        let (sum, overflow) = vx.overflowing_add(vy);

        self.reg_v[x] = sum;

        if overflow {
            self.reg_v[0xF] = 1;
        } else {
            self.reg_v[0xF] = 0;
        }
    }

    fn vx_sub_vy(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x];
        let vy = self.reg_v[y];

        let (diff, overflow) = vx.overflowing_sub(vy);

        self.reg_v[x] = diff;

        if overflow {
            self.reg_v[0xF] = 0;
        } else {
            self.reg_v[0xF] = 1;
        }
    }

    fn vy_sub_vx(&mut self, x: usize, y: usize) {
        let vx = self.reg_v[x];
        let vy = self.reg_v[y];

        let (diff, overflow) = vy.overflowing_sub(vx);
        self.reg_v[x] = diff;

        if overflow {
            self.reg_v[0xF] = 0;
        } else {
            self.reg_v[0xF] = 1;
        }
    }

    fn shift_left(&mut self, x: usize, y: usize) {
        if !self.config.flag_set(ConfigFlags::Shift) {
            self.reg_v[x] = self.reg_v[y]
        }

        let vx = self.reg_v[x];
        self.reg_v[x] = vx << 1;
        self.reg_v[0xF] = (vx & 0b1000_0000) >> 7;
    }

    fn shift_right(&mut self, x: usize, y: usize) {
        if !self.config.flag_set(ConfigFlags::Shift) {
            self.reg_v[x] = self.reg_v[y]
        }

        let vx = self.reg_v[x];
        self.reg_v[x] = vx >> 1;
        self.reg_v[0xF] = vx & 0b0000_0001;
    }

    fn jump_offset(&mut self, x: usize, nnn: usize) {
        self.pc = if !self.config.flag_set(ConfigFlags::JumpWithOffset) {
            let v0 = self.reg_v[0] as usize;
            nnn + v0
        } else {
            let vx = self.reg_v[x] as usize;
            nnn + vx
        };
    }

    fn random(&mut self, x: usize, nn: usize) {
        let mut rng = rand::thread_rng();
        self.reg_v[x] = rng.gen::<u8>() & (nn as u8);
    }

    fn skip_if_down(&mut self, x: usize) {
        let vx: u8 = self.reg_v[x];

        if self.input.check_key_pressed(vx) {
            self.pc += 2
        }
    }

    fn skip_if_up(&mut self, x: usize) {
        let vx: u8 = self.reg_v[x];

        if self.input.check_key_released(vx) {
            self.pc += 2
        }
    }

    fn get_delay_timer(&mut self, x: usize) {
        self.reg_v[x] = self.delay_timer;
    }

    fn set_delay_timer(&mut self, x: usize) {
        self.delay_timer = self.reg_v[x]
    }

    fn set_sound_timer(&mut self, x: usize) {
        self.sound_timer = self.reg_v[x]
    }

    fn add_to_index(&mut self, x: usize) {
        let vx = self.reg_v[x] as usize;
        let wrapped: bool;
        (self.reg_i, wrapped) = self.reg_i.overflowing_add(vx as u16);

        if self.config.flag_set(ConfigFlags::DontIndexOverflow) {
            if wrapped {
                self.reg_v[0xF] = 1;
            } else {
                self.reg_v[0xF] = 0;
            }
        }
    }

    fn get_key(&mut self, x: usize) {
        if !self.input.any_key_pressed() {
            self.pc -= 2
        } else {
            let i = self.input.get_key_pressed();
            self.reg_v[x] = i;
        }
    }

    fn font_character(&mut self, x: usize) {
        let character = self.reg_v[x];

        let addr = FONT_ADDR + character as usize;

        self.reg_i = addr as u16;
    }

    fn bcd_conversion(&mut self, x: usize) {
        let vx = self.reg_v[x] as f32;

        let hundreds = (vx / 100.0).floor() as u8;
        let tens = ((vx / 10.0) % 10.0).floor() as u8;
        let ones = (vx % 10.0) as u8;

        self.memory[self.reg_i as usize] = hundreds;
        self.memory[(self.reg_i + 1) as usize] = tens;
        self.memory[(self.reg_i + 2) as usize] = ones;
    }

    fn store_mem(&mut self, x: usize) {
        for offset in 0..=x {
            if !self.config.flag_set(ConfigFlags::StoreLoadMem) {
                let value = self.reg_v[offset];
                self.memory[self.reg_i as usize] = value;
                self.reg_i += 1;
            } else {
                let value = self.reg_v[offset];
                let addr = self.reg_i + offset as u16;
                self.memory[addr as usize] = value;
            }
        }
    }

    fn load_mem(&mut self, x: usize) {
        for offset in 0..=x {
            if !self.config.flag_set(ConfigFlags::StoreLoadMem) {
                let value = self.memory[self.reg_i as usize];
                self.reg_v[offset] = value;
                self.reg_i += 1;
            } else {
                let addr = self.reg_i + offset as u16;
                let value = self.memory[addr as usize];
                self.reg_v[offset] = value;
            }
        }
    }

    fn display(&mut self, x: usize, y: usize, n: usize) {
        self.reg_v[0x0f] = 0;
        for byte in 0..n {
            let y = (self.reg_v[y] as usize + byte) % Y_PIXELS as usize;
            for bit in 0..8 {
                let x = (self.reg_v[x] as usize + bit) % X_PIXELS as usize;
                let color = (self.memory[(self.reg_i + byte as u16) as usize] >> (7 - bit)) & 1;
                let vram_addr = (y * X_PIXELS as usize * 3) + (x * 3);
                let new_color = (self.vram[vram_addr] / ON) ^ color;
                self.reg_v[0x0f] |= color & (self.vram[vram_addr] / ON);
                self.vram[vram_addr] = new_color * ON;
                self.vram[vram_addr + 1] = new_color * ON;
                self.vram[vram_addr + 2] = new_color * ON;
            }
        }
        self.update_screen = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod shifts {
        use super::*;

        #[test]
        fn test_shift_mode() {
            let x = 0;
            let y = 1;

            let config = Config::default();
            let mut cpu = CPU::new(config);

            cpu.reg_v[y] = 0b0000_1000;
            cpu.reg_v[x] = 0b0100_0000;

            cpu.shift_left(x, y);

            assert_eq!(cpu.reg_v[x], 0b1000_0000);

            let config = Config::from(ConfigFlags::Shift as u8);
            let mut cpu = CPU::new(config);

            cpu.reg_v[y] = 0b0000_1000;
            cpu.reg_v[x] = 0b0100_0000;

            cpu.shift_left(x, y);

            assert_eq!(cpu.reg_v[x], 0b0001_0000);
        }

        #[test]
        fn test_shift_left() {
            let config = Config::default();
            let mut cpu = CPU::new(config);

            let x = 5;
            let y = 0;

            cpu.reg_v[x] = 0b0101_0101;

            cpu.shift_left(x, y);

            assert_eq!(cpu.reg_v[x], 0b1010_1010);
            assert_eq!(cpu.reg_v[0xF], 0);

            cpu.shift_left(x, y);

            assert_eq!(cpu.reg_v[x], 0b0101_0100);
            assert_eq!(cpu.reg_v[0xF], 1);
        }

        #[test]
        fn test_shift_right() {
            let config = Config::default();
            let mut cpu = CPU::new(config);

            let x = 5;
            let y = 0;

            cpu.reg_v[x] = 0b0011_0010;

            cpu.shift_right(x, y);

            assert_eq!(cpu.reg_v[x], 0b0001_1001);
            assert_eq!(cpu.reg_v[0xF], 0);

            cpu.shift_right(x, y);

            assert_eq!(cpu.reg_v[x], 0b0000_1100);
            assert_eq!(cpu.reg_v[0xF], 1);
        }
    }

}
