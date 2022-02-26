use std::fmt;

use super::{register::Register, register_with_offset::RegisterWithOffset};
#[derive(Debug, PartialEq)]
pub enum Operand {
    Ident(String),
    Int(i64),
    RegisterWithOffset(RegisterWithOffset),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operand::Ident(s) => s.clone(),
            Operand::Int(n) => format!("${n}"),
            Operand::RegisterWithOffset(inner) => inner.to_string(),
        };
        write!(f, "{s}")
    }
}

macro_rules! impl_from_operand {
    ($from_ty:ty => Ident) => {
        impl From<$from_ty> for Operand {
            fn from(v: $from_ty) -> Self {
                Self::Ident(v.to_string())
            }
        }
    };
    ($from_ty:ty => RegisterWithOffset) => {
        impl From<$from_ty> for Operand {
            fn from(v: $from_ty) -> Self {
                Self::RegisterWithOffset(
                    crate::go_assembly::register_with_offset::RegisterWithOffset::from(v),
                )
            }
        }
    };
    ($from_ty:ty => $variant:ident) => {
        impl From<$from_ty> for Operand {
            fn from(v: $from_ty) -> Self {
                Self::$variant(v)
            }
        }
    };
}

impl_from_operand!(i64 => Int);
impl_from_operand!(&str => Ident);
impl_from_operand!(String => Ident);
impl_from_operand!(RegisterWithOffset => RegisterWithOffset);
impl_from_operand!(Register => RegisterWithOffset);

#[macro_export]
macro_rules! operand {
    ($offset:expr => $register_variant:ident) => {
        $crate::go_assembly::operand::Operand::RegisterWithOffset(crate::register_with_offset!($offset => $register_variant))
    };
    ($register:ident) => {
       $crate::go_assembly::operand::Operand::RegisterWithOffset(crate::register_with_offset!($register))
    };
    ($expr:expr) => {
        $crate::go_assembly::operand::Operand::from($expr)
    };
}

#[cfg(test)]
mod snapshots {
    use super::*;
    use insta::assert_display_snapshot;

    macro_rules! insta_test {
        ($testname:ident: $($testcases:expr),+) => {
            #[test]
            fn $testname() {
                $(assert_display_snapshot!($testcases);)+
            }
        };
    }

    insta_test!(operand: operand!(AX), operand!(16=>SP), operand!(1));
}
