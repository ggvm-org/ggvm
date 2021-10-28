#[derive(Debug, PartialEq)]
pub enum Instruction {
    Pushq(Operand),
    Popq(Operand),
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    AX,
    BX,
    SP,
    Int(i64),
}

use Instruction::*;
use Operand::*;

impl From<i64> for Operand {
    fn from(v: i64) -> Self {
        Operand::Int(v)
    }
}

macro_rules! pushq {
    ($operand:tt) => {
        Pushq(operand!($operand))
    };
}

macro_rules! operand {
    (AX) => {
        Operand::AX
    };
    ($expr:expr) => {
        Operand::from($expr)
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        Instruction::{self, *},
        Operand::{self, *},
    };

    #[test]
    fn operand() {
        assert_eq!(operand!(AX), AX);
        assert_eq!(operand!(1), Int(1));
    }

    #[test]
    fn pushq() {
        assert_eq!(pushq!(AX), Pushq(AX));
        assert_eq!(pushq!(1), Pushq(Int(1)));
    }
}
