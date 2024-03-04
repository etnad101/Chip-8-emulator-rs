/*
Used to specify behaviour for specific functionns.
"Modern" behavior is used by default if NONE is used
*/

use std::ops::BitOr;

pub enum InstructionFlags {
    Shift = 0b1000_0000,
    JumpWithOffset = 0b0100_0000,
    StoreLoadMem = 0b0010_0000,
}

impl BitOr for InstructionFlags {
    type Output = u8;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<InstructionFlags> for u8 {
    type Output = Self;
    fn bitor(self, rhs: InstructionFlags) -> Self::Output {
        self | rhs as u8
    }
}

pub struct Config {
    flags: u8,
}

impl Config {
    pub fn default() -> Self {
        Config { flags: 0 }
    }

    pub fn from(flags: u8) -> Self {
        Config { flags }
    }

    pub fn flag_set(&self, flag: InstructionFlags) -> bool {
        let res = self.flags & flag as u8;
        res > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flag_set() {
        let c = Config::default();

        assert_eq!(c.flag_set(InstructionFlags::Shift), false);
        assert_eq!(c.flag_set(InstructionFlags::JumpWithOffset), false);
        assert_eq!(c.flag_set(InstructionFlags::StoreLoadMem), false);

        let c = Config::from(InstructionFlags::Shift as u8);

        assert_eq!(c.flag_set(InstructionFlags::Shift), true);
        assert_eq!(c.flag_set(InstructionFlags::JumpWithOffset), false);
        assert_eq!(c.flag_set(InstructionFlags::StoreLoadMem), false);

        let c = Config::from(InstructionFlags::Shift | InstructionFlags::JumpWithOffset);

        assert_eq!(c.flag_set(InstructionFlags::Shift), true);
        assert_eq!(c.flag_set(InstructionFlags::JumpWithOffset), true);
        assert_eq!(c.flag_set(InstructionFlags::StoreLoadMem), false);

        let c = Config::from(
            InstructionFlags::Shift
                | InstructionFlags::StoreLoadMem
                | InstructionFlags::JumpWithOffset,
        );

        assert_eq!(c.flag_set(InstructionFlags::Shift), true);
        assert_eq!(c.flag_set(InstructionFlags::JumpWithOffset), true);
        assert_eq!(c.flag_set(InstructionFlags::StoreLoadMem), true);
    }
}
