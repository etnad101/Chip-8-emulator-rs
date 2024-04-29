use std::fs;

pub enum ProgramType {
    Test(u8),
    Path(String),
}

pub struct Program {
    pub bytes: Vec<u8>,
}

impl Program {
    pub fn new(path: ProgramType) -> Self {
        let program_path = match path {
            ProgramType::Test(test_num) => match test_num {
                1 => "roms/tests/1-chip8-logo.ch8",
                2 => "roms/tests/2-ibm-logo.ch8",
                3 => "roms/tests/3-corax+.ch8",
                4 => "roms/tests/4-flags.ch8",
                5 => "roms/tests/5-quirks.ch8",
                6 => "roms/tests/6-keypad.ch8",
                7 => "roms/tests/7-beep.ch8",
                8 => "roms/tests/8-scrolling.ch8",
                _ => panic!("The test program must be from 1-8"),
            }
            .to_string(),
            ProgramType::Path(p) => p,
        };

        let bytes = fs::read(program_path).expect("Unable to read file");

        Program { bytes }
    }
}
