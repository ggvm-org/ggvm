use std::fmt;

use self::jmp_target::JmpTarget;

use super::operand::Operand;

#[macro_use]
pub mod jmp_target;

#[derive(Debug, PartialEq)]
pub(crate) enum Directive {
    Text { package: String, name: String },
    Subq(Operand, Operand),
    Leaq(Operand, Operand),
    Movq(Operand, Operand),
    Call { package: String, name: String },
    Addq(Operand, Operand),
    Ret,

    // CMPQ	SP, 16(R14)
    Cmpq(Operand, Operand),
    // PCDATA	$0, $-2
    PCData(Operand, Operand),

    // epi:
    Label(String),
    // NOP
    Nop,
    // JMP body
    Jmp(JmpTarget),
    // Jls	epi
    Jls(JmpTarget),
}

#[macro_export]
macro_rules! directive {
    (NOP) => {
        Directive::Nop
    };
    (RET) => {
        Directive::Ret
    };
    (PCDATA $left:expr, $right:expr) => {
        PCDATA!($left, $right)
    };
    (CALL $package:ident.$name:ident) => {
        CALL!($package.$name)
    };
    (JMP @$target:tt) => {
        JMP!(@$target)
    };
    (JMP $target:tt) => {
        JMP!($target)
    };
    (@$label_name:ident) => {
        Directive::Label(stringify!($label_name))
    };
}

#[macro_export]
macro_rules! PCDATA {
    ($left:expr, $right:expr) => {
        Directive::PCData(operand!($left), operand!($right))
    };
}

#[macro_export]
macro_rules! ADDQ {
    ($left_op:tt, $right_op:tt) => {
        Directive::Addq(operand!($left_op), operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_op:tt) => {
        Directive::Addq(operand!($left_offset => $left_op), operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
        Directive::Addq(operand!($left_offset => $left_op), operand!($right_offset => $right_op))
    };
    ($left_op:tt, $right_offset:tt => $right_op:tt) => {
        Directive::Addq(operand!($left_op), operand!($right_offset => $right_op))
    };
}

#[macro_export]
macro_rules! CALL {
    ($package:ident.$name:ident) => {
        Directive::Call {
            package: stringify!($package).to_string(),
            name: stringify!($name).to_string(),
        }
    };
}

macro_rules! define_jmp_macro {
    ($macro_name:ident, $variant:ident) => {
        #[macro_export]
        macro_rules! $macro_name {
            ($target:expr) => {
                Directive::$variant(JmpTarget::from($target))
            };
            (@$label:ident) => {
                Directive::$variant(JmpTarget::from(stringify!($label)))
            };
        }
    };
}

define_jmp_macro!(JMP, Jmp);
define_jmp_macro!(JLS, Jls);

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text { package, name } => format!("TEXT	{}.{}(SB), 4, $0-0", package, name),
            Self::Subq(left, right) => format!("SUBQ	{}, {}", left, right),
            Self::Call { package, name } => format!("CALL    {package}·{name}(SB)"),
            Self::Addq(left, right) => format!("ADDQ	{}, {}", left, right),
            Self::Movq(left, right) => format!("MOVQ	{}, {}", left, right),
            Self::Cmpq(left, right) => format!("Cmpq	{}, {}", left, right),
            Self::Jls(target) => format!("Jls	{}", target),
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

    insta_test!(directive_jmp: directive!(JMP 2), directive!(JMP @body));

    insta_test!(directive_call: directive!(CALL main.run));

    insta_test!(nop: directive!(NOP));
    insta_test!(call: CALL!(main.run));

    const TEST_JMP_TARGET_VAR: &str = "AAAAA";
    insta_test!(
        jmp: JMP!(33),
        JMP!("epi"),
        JMP!(TEST_JMP_TARGET_VAR),
        JMP!(@body)
    );

    insta_test!(
        jls: JLS!(33),
        JLS!("epi"),
        JLS!(TEST_JMP_TARGET_VAR),
        JLS!(@body)
    );

    insta_test!(
        addq: ADDQ!(AX, 1),
        ADDQ!(16=>AX, 1),
        ADDQ!(1, 16=>AX),
        ADDQ!(16=>AX, 16=>SP)
    );

    insta_test!(pcdata: PCDATA!(1, 2));
    insta_test!(directive_pcdata: directive!(PCDATA 1, 2));
}
