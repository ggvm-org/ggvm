use crate::{
    go_assembly::{GoAssembly, GoAssemblyKind},
    Func, Instruction, Statement,
};

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
