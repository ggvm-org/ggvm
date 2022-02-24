use std::ops::{Deref, DerefMut};

use crate::Operand;

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

impl ToString for AsmOperand {
    fn to_string(&self) -> String {
        match self {
            AsmOperand::Ident(s) => s.clone(),
            AsmOperand::Int(n) => format!("${n}"),
            AsmOperand::RegisterWithOffset(inner) => inner.to_string(),
            AsmOperand::Register(register) => register.to_string(),
        }
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
