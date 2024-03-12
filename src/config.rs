/*
Used to specify behaviour for specific functionns.
"Modern" behavior is used by default
*/

use std::ops::BitOr;

pub enum ConfigFlags {
    Shift = 0b1000_0000,
    JumpWithOffset = 0b0100_0000,
    StoreLoadMem = 0b0010_0000,
    DontIndexOverflow = 0b0001_0000,
}

impl BitOr for ConfigFlags {
    type Output = u8;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<ConfigFlags> for u8 {
    type Output = Self;
    fn bitor(self, rhs: ConfigFlags) -> Self::Output {
        self | rhs as u8
    }
}

#[derive(Default)]
pub struct Config {
    flags: u8,
}

impl Config {
    pub fn from(flags: u8) -> Self {
        Config { flags }
    }

    pub fn flag_set(&self, flag: ConfigFlags) -> bool {
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

        assert_eq!(c.flag_set(ConfigFlags::Shift), false);
        assert_eq!(c.flag_set(ConfigFlags::JumpWithOffset), false);
        assert_eq!(c.flag_set(ConfigFlags::StoreLoadMem), false);

        let c = Config::from(ConfigFlags::Shift as u8);

        assert_eq!(c.flag_set(ConfigFlags::Shift), true);
        assert_eq!(c.flag_set(ConfigFlags::JumpWithOffset), false);
        assert_eq!(c.flag_set(ConfigFlags::StoreLoadMem), false);

        let c = Config::from(ConfigFlags::Shift | ConfigFlags::JumpWithOffset);

        assert_eq!(c.flag_set(ConfigFlags::Shift), true);
        assert_eq!(c.flag_set(ConfigFlags::JumpWithOffset), true);
        assert_eq!(c.flag_set(ConfigFlags::StoreLoadMem), false);

        let c = Config::from(
            ConfigFlags::Shift | ConfigFlags::StoreLoadMem | ConfigFlags::JumpWithOffset,
        );

        assert_eq!(c.flag_set(ConfigFlags::Shift), true);
        assert_eq!(c.flag_set(ConfigFlags::JumpWithOffset), true);
        assert_eq!(c.flag_set(ConfigFlags::StoreLoadMem), true);
    }
}
