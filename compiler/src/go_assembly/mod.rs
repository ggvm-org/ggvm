use std::ops::{Deref, DerefMut};

use crate::Operand;

#[derive(Debug, PartialEq)]
pub(crate) enum GoAssemblyKind {
    Text { package: String, name: String },
    Subq(Operand, Operand),
    Leaq(Operand, Operand),
    Movq(Operand, Operand),
    Call(Operand),
    Addq(Operand, Operand),
}

#[derive(Debug, PartialEq)]
pub(crate) enum AsmOperand {
    RegisterWithOffset(RegisterWithOffset),
}

impl ToString for AsmOperand {
    fn to_string(&self) -> String {
        match self {
            AsmOperand::RegisterWithOffset(inner) => inner.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RegisterWithOffset {
    offset: usize,
    register: Register,
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
pub(crate) enum Register {
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
            _ => unimplemented!(),
        }
    }
}

pub(crate) struct GoAssembly(pub(crate) Vec<GoAssemblyKind>);

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
