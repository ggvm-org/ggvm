use std::ops::{Deref, DerefMut};

use crate::{Func, Instruction, Statement};

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

pub(crate) fn compile_func(func: Func) -> GoAssembly {
    let package = "main".to_string();
    let name = func.name;
    let mut asm = GoAssembly(vec![GoAssemblyKind::Text { package, name }]);
    func.stmts.into_iter().for_each(|stmt| {
        asm.append(&mut compile_stmt(stmt));
    });
    asm
}

pub(crate) fn compile_stmt(stmt: super::Statement) -> GoAssembly {
    match stmt {
        Statement::Local(op, inst) => unimplemented!(),
        Statement::Inst(inst) => unimplemented!(),
    }
}

pub(crate) fn compile_inst(inst: Instruction) -> GoAssembly {
    match inst {
        Instruction::Add(typ, a, b) => unimplemented!(),
        Instruction::Ret(typ, a) => unimplemented!(),
    }
}
