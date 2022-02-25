use std::{
    fmt,
    ops::{Deref, DerefMut},
};

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
}

#[derive(Debug, PartialEq)]
pub enum AsmOperand {
    Ident(String),
    Int(usize),
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

#[derive(Debug, PartialEq)]
pub struct RegisterWithOffset {
    pub(crate) offset: usize,
    pub(crate) register: Register,
}

impl fmt::Display for RegisterWithOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = {
            let offset = self.offset;
            let register = self.register;
            if offset == 0 {
                register.to_string()
            } else {
                format!("{offset}({register})")
            }
        };
        write!(f, "{s}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    AX,
    CX,
    SP,
    BP,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            &Self::AX => "AX",
            &Self::CX => "CX",
            &Self::SP => "SP",
            &Self::BP => "BP",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for GoAssemblyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text { package, name } => format!("TEXT	{}.{}(SB), 4, $0-0", package, name),
            Self::Subq(left, right) => format!("SUBQ	{}, {}", left, right),
            Self::Call(AsmOperand::Ident(ident)) => format!("CALL    main.{ident}(SB)"),
            Self::Addq(left, right) => format!("ADDQ	{}, {}", left, right),
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
    use crate::go_assembly::{AsmOperand, GoAssemblyKind, Register::*, RegisterWithOffset};
    use insta::assert_display_snapshot;

    #[test]
    fn go_assembly_kind() {
        assert_display_snapshot!(GoAssemblyKind::Text {
            package: "main".to_string(),
            name: "run".to_string()
        });
        assert_display_snapshot!(GoAssemblyKind::Subq(
            AsmOperand::Int(10000),
            AsmOperand::Register(SP)
        ));
        assert_display_snapshot!(GoAssemblyKind::Addq(
            AsmOperand::Int(10000),
            AsmOperand::Register(SP)
        ))
    }

    #[test]
    fn register_with_offset() {
        assert_display_snapshot!(RegisterWithOffset {
            register: AX,
            offset: 8
        });
        assert_display_snapshot!(RegisterWithOffset {
            register: SP,
            offset: 0
        })
    }

    #[test]
    fn asm_operand() {
        assert_display_snapshot!(AsmOperand::Ident("a".to_string()));
        assert_display_snapshot!(AsmOperand::Int(1));
        assert_display_snapshot!(AsmOperand::Register(AX));
    }
}
