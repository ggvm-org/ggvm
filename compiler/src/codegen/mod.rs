use plan9_asm::*;

use crate::{analyze::AnalyzeResult, go_assembly::GoAssembly, Instruction, Operand, Statement};

pub fn compile() -> impl FnOnce(AnalyzeResult) -> GoAssembly {
    compile_func
}

pub(crate) fn compile_func(func: AnalyzeResult) -> GoAssembly {
    let func = func.func;
    let package = "main".to_string();
    let name = func.name;
    let asm = GoAssembly(directives!(
        TEXT main.run;
        SUBQ [10000], [SP];
        MOVQ [BP], [16(SP)];
    ));
    let body = func
        .stmts
        .into_iter()
        .fold(GoAssembly(vec![]), |body, stmt| body + compile_stmt(stmt));

    let epilogue = GoAssembly(directives!(
        MOVQ [16(SP)], [BP];
        ADDQ [10000], [SP];
    ));
    asm + body + epilogue
}

pub(crate) fn compile_stmt(stmt: super::Statement) -> GoAssembly {
    match stmt {
        Statement::Local(op, inst) => unimplemented!(),
        Statement::Inst(inst) => compile_inst(inst),
    }
}

pub(crate) fn compile_inst(inst: Instruction) -> GoAssembly {
    GoAssembly(match inst {
        Instruction::Add(typ, a, b) => unimplemented!(),
        Instruction::Ret(typ, a) => directives!(RET;),
        Instruction::Call(Operand::Var(var)) => directives!(
            CALL {"main"}.{var};
            MOVQ [AX], [8(SP)];
        ),
    })
}
