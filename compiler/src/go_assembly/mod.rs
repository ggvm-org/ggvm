use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use self::{register::Register, register_with_offset::RegisterWithOffset};

mod directive;
pub mod register;
pub mod register_with_offset;

pub use directive::*;
pub use register::*;
pub use register_with_offset::*;

// TODO: rename to Directives
#[derive(Debug, PartialEq)]
pub enum GoAssemblyKind {
    Text { package: String, name: String },
    Subq(AsmOperand, AsmOperand),
    Leaq(AsmOperand, AsmOperand),
    Movq(AsmOperand, AsmOperand),
    Call(AsmOperand),
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
    Jmp(AsmOperand),

    // temporary
    CallWithPkg { package: String, name: String },
}

impl GoAssembly {
    fn new_goroutine_prologue() -> Self {
        Self(vec![
            GoAssemblyKind::Cmpq(
                AsmOperand::Register(Register::SP),
                AsmOperand::RegisterWithOffset(RegisterWithOffset {
                    offset: 16,
                    register: Register::R14,
                }),
            ),
            GoAssemblyKind::PCData(AsmOperand::Int(0), AsmOperand::Int(-2)),
            GoAssemblyKind::JLS(AsmOperand::Ident("epi".to_string())),
            GoAssemblyKind::Label("body".to_string()),
        ])
    }

    fn new_goroutine_epilogue() -> Self {
        // epi:
        // NOP
        // PCDATA	$1, $-1
        // PCDATA	$0, $-2
        // CALL	runtime·morestack_noctxt(SB)
        // PCDATA	$0, $-1
        // JMP	body
        Self(vec![
            GoAssemblyKind::Label("epi".to_string()),
            GoAssemblyKind::Nop,
            GoAssemblyKind::PCData(AsmOperand::Int(1), AsmOperand::Int(-1)),
            GoAssemblyKind::PCData(AsmOperand::Int(0), AsmOperand::Int(-2)),
            GoAssemblyKind::CallWithPkg {
                package: "runtime".to_string(),
                name: "morestack_noctxt".to_string(),
            },
            GoAssemblyKind::PCData(AsmOperand::Int(0), AsmOperand::Int(-1)),
            GoAssemblyKind::Jmp(AsmOperand::Ident("body".to_string())),
        ])
    }
}

#[derive(Debug, PartialEq)]
pub enum AsmOperand {
    Ident(String),
    Int(i64),
    RegisterWithOffset(RegisterWithOffset),
    Register(Register),
}

impl fmt::Display for AsmOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AsmOperand::Ident(s) => s.clone(),
            AsmOperand::Int(n) => format!("${n}"),
            AsmOperand::RegisterWithOffset(inner) => inner.to_string(),
            AsmOperand::Register(register) => register.to_string(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for GoAssemblyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text { package, name } => format!("TEXT	{}.{}(SB), 4, $0-0", package, name),
            Self::Subq(left, right) => format!("SUBQ	{}, {}", left, right),
            Self::Call(AsmOperand::Ident(ident)) => format!("CALL    main·{ident}(SB)"),
            Self::Addq(left, right) => format!("ADDQ	{}, {}", left, right),
            Self::Movq(left, right) => format!("MOVQ	{}, {}", left, right),
            Self::Cmpq(left, right) => format!("Cmpq	{}, {}", left, right),
            Self::JLS(target) => format!("JLS	{}", target),
            Self::PCData(left, right) => format!("PCDATA {}, {}", left, right),
            Self::Label(label_name) => format!("{}:", label_name),
            Self::Jmp(target) => format!("JMP	{}", target),
            Self::Nop => "NOP".to_string(),
            Self::CallWithPkg { package, name } => format!("CALL {package}·{name}(SB)"),
            _ => unimplemented!(),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub struct GoAssembly(pub(crate) Vec<GoAssemblyKind>);

impl Deref for GoAssembly {
    type Target = Vec<GoAssemblyKind>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GoAssembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for GoAssembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stmts_str = self.iter().fold(String::new(), |asm_body_str, goasm| {
            format!("{}\n{}", asm_body_str, goasm.to_string())
        });
        write!(f, "{stmts_str}")
    }
}

#[cfg(test)]
mod insta {
    use crate::go_assembly::{
        AsmOperand, GoAssembly, GoAssemblyKind, Register::*, RegisterWithOffset,
    };
    use insta::assert_display_snapshot;

    macro_rules! insta_test {
        ($testname:ident: $($testcases:expr),+) => {
            #[test]
            fn $testname() {
                $(assert_display_snapshot!($testcases);)+
            }
        };
    }

    insta_test!(go_routine_prologue: GoAssembly::new_goroutine_prologue());

    insta_test!(go_routine_epilogue: GoAssembly::new_goroutine_epilogue());

    insta_test!(
        go_assembly:
            GoAssembly(
                vec![
                    GoAssemblyKind::Text {
                        package: "main".to_string(),
                        name: "run".to_string()
                    },
                    GoAssemblyKind::Subq(AsmOperand::Int(10000), AsmOperand::Register(SP)),
                    GoAssemblyKind::Movq(
                        AsmOperand::Register(BP),
                        AsmOperand::RegisterWithOffset(RegisterWithOffset {
                            offset: 16,
                            register: SP
                        })
                    ),
                    GoAssemblyKind::Call(AsmOperand::Ident("rantIntn".to_string())),
                    GoAssemblyKind::Movq(
                        AsmOperand::RegisterWithOffset(RegisterWithOffset {
                            offset: 16,
                            register: SP
                        }),
                        AsmOperand::Register(BP),
                    ),
                    GoAssemblyKind::Addq(AsmOperand::Int(10000), AsmOperand::Register(SP))
                ],
            )
    );

    insta_test!(
        go_assembly_kind: GoAssemblyKind::Text {
            package: "main".to_string(),
            name: "run".to_string()
        },
        GoAssemblyKind::Subq(
            AsmOperand::Int(10000),
            AsmOperand::Register(SP)
        ),
        GoAssemblyKind::Addq(
            AsmOperand::Int(10000),
            AsmOperand::Register(SP)
        ),
        GoAssemblyKind::JLS(AsmOperand::Int(100)),
        GoAssemblyKind::JLS(AsmOperand::Ident("a".to_string())),
        GoAssemblyKind::Cmpq(AsmOperand::Register(SP), AsmOperand::RegisterWithOffset(RegisterWithOffset{
            offset: 16,
            register: R14
        })),
        GoAssemblyKind::PCData(AsmOperand::Int(1), AsmOperand::Int(2)),
        GoAssemblyKind::Label("epi".to_string())
    );

    insta_test!(
        register_with_offset: RegisterWithOffset {
            register: AX,
            offset: 8
        },
        RegisterWithOffset {
            register: SP,
            offset: 0
        }
    );

    insta_test!(
        asm_operand: AsmOperand::Ident("a".to_string()),
        AsmOperand::Int(1),
        AsmOperand::Register(AX)
    );
}
