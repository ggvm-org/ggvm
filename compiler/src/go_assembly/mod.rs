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
