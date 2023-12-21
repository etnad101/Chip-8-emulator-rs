use crate::settings::*;

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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct CPU {
    pub memory: [u8; MEM_SIZE],
    pub pc: usize,
    pub reg_i: usize,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub reg_v: [u8; 16],
    pub vram: [u8; 64 * 32 * 3],
    pub update_screen: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; MEM_SIZE],
            pc: PROGRAM_START,
            reg_i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            reg_v: [0; 16],
            vram: [0;64 * 32 * 3],
            update_screen: true,
        }
    }

    // Debug 
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
        for i in 0..program.len() {
            self.memory[PROGRAM_START + i] = program[i];
        }
    }
    
    // Main loop

    fn fetch(&mut self) -> (u8, u8, u8, u8) {
        let first_byte = self.memory[self.pc as usize];
        let second_byte =  self.memory[(self.pc + 1) as usize];
        let instruction: u16 = (first_byte as u16) << 8 | second_byte as u16;
        self.pc += 2;

        (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            ((instruction & 0x000F)) as u8
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
                (0x00, 0x00, 0x0e, 0x00) => self.clear_screen(),
                (0x00, 0x00, 0x0e, 0x0e) => self.sub_return(),
                (0x0d, _, _, _) => self.display(x, y, n),
                (0x06, _, _, _) => self.set_reg_v(x, nn),
                (0x07, _, _, _) => self.add_reg_v(x, nn),
                (0x0a, _, _, _) => self.set_index(nnn),
                (0x02, _, _, _) => self.subroutine(nnn),
                (0x01, _, _, _) => self.jump(nnn),
                _ => todo!("Unimplemented opcode: {:#x}, {:#x}, {:#x}, {:#x} @ pc = {:#x}", instruction.0, instruction.1, instruction.2, instruction.3 , self.pc - 2)
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
        self.reg_i = nnn;
    }

    fn set_reg_v(&mut self, x: usize, nn: usize) {
        self.reg_v[x as usize] = nn as u8;
    }

    fn add_reg_v(&mut self, x: usize, nn: usize) {
        let mut value = self.reg_v[x as usize];
        value = value.wrapping_add(nn as u8);
        self.reg_v[x as usize] = value;
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

    fn display(&mut self, x: usize, y: usize, n: usize) {
        // self.reg_v[0x0F] = 0;
        
        // for byte in 0..n {
        //     let y_coord = self.reg_v[(y as usize) + byte as usize] % Y_PIXELS as u8;
        //     for bit in 0..8 {
        //         let x_coord = self.reg_v[(x as usize) + bit as usize] % X_PIXELS as u8;
        //         let color: u8 = (self.memory[(self.reg_i as usize) + (byte as usize)] >> (7-bit)) & 1;
        //         let vram_addr = (y_coord as usize * X_PIXELS as usize * 3) + x_coord  as usize * 3;
        //         let state = (self.vram[vram_addr] >> 7) ^ color;
        //         self.vram[vram_addr] = ON * state;
        //         self.vram[vram_addr + 1] = ON * state;
        //         self.vram[vram_addr + 2] = ON * state;
        //     }
        // }

        // self.update_screen = true;

        self.reg_v[0x0f] = 0;
        for byte in 0..n {
            let y = (self.reg_v[y] as usize + byte) % X_PIXELS as usize;
            for bit in 0..8 {
                let x = (self.reg_v[x] as usize + bit) % X_PIXELS as usize;
                let color = (self.memory[self.reg_i + byte] >> (7 - bit)) & 1;
                let addr = (3 * y * Y_PIXELS as usize) + x * 3;
                if self.vram[addr] == 0 {
                    color =
                }
                self.vram[addr] ^= color;

            }
        }
        self.update_screen = true;
    }

}
