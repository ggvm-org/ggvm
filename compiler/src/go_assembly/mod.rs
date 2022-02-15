use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
pub(crate) enum GoAssemblyKind {
    Text { package: String, name: String },
    // Pushq(Operand),
    // Popq(Operand),
    // Movq(Expression),
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
