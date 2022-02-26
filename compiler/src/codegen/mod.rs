use plan9_asm::*;

use crate::{analyze::AnalyzeResult, go_assembly::GoAssembly, Instruction, Operand, Statement};

pub fn compile() -> impl FnOnce(AnalyzeResult) -> GoAssembly {
    compile_func
}

pub(crate) fn compile_func(func: AnalyzeResult) -> GoAssembly {
    let func = func.func;
    let package = "main".to_string();
    let name = func.name;
    operand!(AX);
    let mut asm = GoAssembly(vec![
        Directive::Text { package, name },
        SUBQ!(10000, SP),
        MOVQ!(BP, 16=>SP),
    ]);
    let body = GoAssembly(func.stmts.into_iter().fold(vec![], |mut body, stmt| {
        body.append(&mut compile_stmt(stmt).0);
        body
    }));

    let epilogue = GoAssembly(vec![MOVQ!(16=>SP, BP), ADDQ!(10000, SP)]);
    asm.extend(body.0);
    asm.extend(epilogue.0);
    asm
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
        Instruction::Ret(typ, a) => vec![directive!(RET)],
        Instruction::Call(Operand::Var(var)) => {
            vec![
                Directive::Call {
                    package: "main".to_string(),
                    name: var,
                },
                MOVQ!(AX, 8 => SP),
            ]
        }
    })
}
