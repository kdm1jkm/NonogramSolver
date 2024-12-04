use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Block = 0b10,
    Blank = 0b01,
    Crash = 0b11,
    Unknown = 0b00,
}

impl std::ops::BitOr for Cell {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self as u8) | (rhs as u8) {
            0b10 => Cell::Block,
            0b01 => Cell::Blank,
            0b11 => Cell::Crash,
            0b00 => Cell::Unknown,
            _ => unreachable!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Block => "██",
            Cell::Blank => "  ",
            Cell::Crash => "CC",
            Cell::Unknown => "??",
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Cell::*;

    #[test]
    fn test_bitor_1() {
        assert_eq!(Block | Blank, Crash);
    }

    #[test]
    fn test_bitor_2() {
        assert_eq!(Blank | Unknown, Blank);
    }

    #[test]
    fn test_bitor_3() {
        assert_eq!(Unknown | Unknown, Unknown);
    }

    #[test]
    fn test_bitor_4() {
        assert_eq!(Block | Block, Block);
    }

    #[test]
    fn test_bitor_5() {
        assert_eq!(Blank | Crash, Crash);
    }

    #[test]
    fn test_bitor_6() {
        assert_eq!(Crash | Unknown, Crash);
    }
}
