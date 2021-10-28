#[derive(Debug, PartialEq)]
pub enum Instruction {
    Text { package: String, name: String },
    Pushq(Operand),
    Popq(Operand),
    Movq(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Operand(Operand),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pushq(operand) => write!(f, "PUSHQ {}\n", operand),
            Popq(operand) => write!(f, "POPQ {}\n", operand),
            Text { package, name } => write!(f, "TEXT {}·{}(SB), $0", package, name),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    AX,
    BX,
    SP,
    Int(i64),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AX => write!(f, "AX"),
            BX => write!(f, "BX"),
            SP => write!(f, "SP"),
            Int(i) => write!(f, "{}", i),
        }
    }
}

use Instruction::*;
use Operand::*;

impl From<i64> for Operand {
    fn from(v: i64) -> Self {
        Operand::Int(v)
    }
}

#[macro_export]
macro_rules! pushq {
    ($operand:tt) => {
        Pushq(operand!($operand))
    };
}

#[macro_export]
macro_rules! popq {
    ($operand:tt) => {
        Popq(operand!($operand))
    };
}

#[macro_export]
macro_rules! text {
    ($name:ident) => {
        format!(
            "{}",
            $crate::Instruction::Text {
                package: "main".to_string(),
                name: stringify!($name).to_string(),
            }
        )
    };
}

// movq!(BP, 8(SP))
// movq!(n, ${align}(SP)\n", n, align)

macro_rules! movq {
    () => {};
}

macro_rules! operand {
    (AX) => {
        AX
    };
    ($expr:expr) => {
        Operand::from($expr)
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        Instruction::*,
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

    #[test]
    fn text() {
        assert_eq!(
            text!(run),
            format!(
                "{}",
                Text {
                    package: "main".to_string(),
                    name: "run".to_string()
                }
            )
        )
    }
}
