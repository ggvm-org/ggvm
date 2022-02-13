use crate::{Instruction, Statement};

#[derive(Debug, PartialEq)]
pub(crate) enum GoAssemblyKind {
    Text { package: String, name: String },
    // Pushq(Operand),
    // Popq(Operand),
    // Movq(Expression),
}

pub(crate) struct GoAssembly(pub(crate) Vec<GoAssemblyKind>);

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
