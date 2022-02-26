use std::fmt;

use super::AsmOperand;

#[derive(Debug, PartialEq)]
pub(crate) enum Directive {
    Text { package: String, name: String },
    Subq(AsmOperand, AsmOperand),
    Leaq(AsmOperand, AsmOperand),
    Movq(AsmOperand, AsmOperand),
    Call { package: String, name: String },
    Addq(AsmOperand, AsmOperand),
    Ret,

    // CMPQ	SP, 16(R14)
    Cmpq(AsmOperand, AsmOperand),
    // PCDATA	$0, $-2
    PCData(AsmOperand, AsmOperand),
    // JLS	epi
    JLS(AsmOperand),
    // epi:
    Label(String),
    // NOP
    Nop,
    // JMP body
    Jmp(JmpTarget),
}

#[derive(Debug, PartialEq)]
pub(crate) enum JmpTarget {
    Addr(i64),
    Label(String),
}

macro_rules! impl_from_jmp {
    ($from_ty:ty => Label) => {
        impl From<$from_ty> for JmpTarget {
            fn from(v: $from_ty) -> Self {
                Self::Label(v.to_string())
            }
        }
    };
    ($from_ty:ty => Addr) => {
        impl From<$from_ty> for JmpTarget {
            fn from(v: $from_ty) -> Self {
                Self::Addr(i64::from(v))
            }
        }
    };
}

impl_from_jmp!(&str => Label);
impl_from_jmp!(String => Label);
impl_from_jmp!(u8 => Addr);

// impl From<i64> for JmpTarget {}

macro_rules! directive {
    (NOP) => {
        Directive::Nop
    };
    (RET) => {
        Directive::Ret
    };
    (CALL $package:ident.$name:ident) => {
        CALL!($package:ident.$name:ident)
    };
    (JMP $target:tt) => {
        JMP!($tt)
    };
    (@$label_name:ident) => {
        Directive::Label(stringify!($label_name))
    };
}

macro_rules! CALL {
    ($package:ident.$name:ident) => {
        Directive::Call {
            package: stringify!($package).to_string(),
            name: stringify!($name).to_string(),
        }
    };
}

macro_rules! JMP {
    ($target:expr) => {
        Directive::Jmp(JmpTarget::from($target))
    };
    (@$label:ident) => {
        Directive::Jmp(JmpTarget::from(stringify!($label)))
    };
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text { package, name } => format!("TEXT	{}.{}(SB), 4, $0-0", package, name),
            Self::Subq(left, right) => format!("SUBQ	{}, {}", left, right),
            Self::Call { package, name } => format!("CALL    {package}·{name}(SB)"),
            Self::Addq(left, right) => format!("ADDQ	{}, {}", left, right),
            Self::Movq(left, right) => format!("MOVQ	{}, {}", left, right),
            Self::Cmpq(left, right) => format!("Cmpq	{}, {}", left, right),
            Self::JLS(target) => format!("JLS	{}", target),
            Self::PCData(left, right) => format!("PCDATA {}, {}", left, right),
            Self::Label(label_name) => format!("{}:", label_name),
            Self::Jmp(target) => format!("JMP	{}", target),
            Self::Nop => "NOP".to_string(),
            _ => unimplemented!(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for JmpTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Addr(s) => s.to_string(),
            Self::Label(l) => l.to_string(),
        };
        write!(f, "{s}")
    }
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

    insta_test!(nop: directive!(NOP));
    insta_test!(call: CALL!(main.run));

    const TEST_JMP_TARGET_VAR: &str = "AAAAA";
    insta_test!(
        jmp: JMP!(33),
        JMP!("epi"),
        JMP!(TEST_JMP_TARGET_VAR),
        JMP!(@body)
    );
}
