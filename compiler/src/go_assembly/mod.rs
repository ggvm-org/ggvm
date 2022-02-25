use std::{
    fmt,
    ops::{Deref, DerefMut},
};

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

impl ToString for RegisterWithOffset {
    fn to_string(&self) -> String {
        let offset = self.offset;
        let register = self.register;
        if offset == 0 {
            register.to_string()
        } else {
            "{offset}(register)".to_string()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    AX,
    CX,
    SP,
    BP,
}

impl ToString for Register {
    fn to_string(&self) -> String {
        match self {
            &Self::AX => "AX",
            &Self::CX => "CX",
            &Self::SP => "SP",
            &Self::BP => "BP",
        }
        .to_string()
    }
}

impl ToString for GoAssemblyKind {
    fn to_string(&self) -> String {
        match self {
            Self::Text { package, name } => unimplemented!(),
            Self::Subq(left, right) => unimplemented!(),
            Self::Call(AsmOperand::Ident(ident)) => format!("CALL    main.{ident}(SB)"),
            _ => unimplemented!(),
        }
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
mod tests {
    use crate::go_assembly::{AsmOperand, Register};

    #[test]
    fn insta_asmoperand() {
        insta::assert_display_snapshot!(AsmOperand::Ident("a".to_string()));
        insta::assert_display_snapshot!(AsmOperand::Int(1));
        insta::assert_display_snapshot!(AsmOperand::Register(Register::AX));
    }
}
