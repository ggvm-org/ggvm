use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    AX,
    CX,
    SP,
    BP,
    R14,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            &Self::AX => "AX",
            &Self::CX => "CX",
            &Self::SP => "SP",
            &Self::BP => "BP",
            &Self::R14 => "R14",
        };
        write!(f, "{s}")
    }
}

#[macro_export]
macro_rules! register {
    ($variant:ident) => {
        Register::$variant
    };
}
