use crate::{
    analyze::AnalyzeResult,
    go_assembly::{
        register::Register, register_with_offset::RegisterWithOffset, AsmOperand, GoAssembly,
        GoAssemblyKind,
    },
    Func, Instruction, Operand, Statement,
};

pub fn compile() -> impl FnOnce(AnalyzeResult) -> GoAssembly {
    compile_func
}

pub(crate) fn compile_func(func: AnalyzeResult) -> GoAssembly {
    let func = func.func;
    let package = "main".to_string();
    let name = func.name;
    let mut asm = GoAssembly(vec![
        // TEXT    {package}.{name}(SB), ABIInternal, $24-0
        GoAssemblyKind::Text { package, name },
        // SUBQ    $10000, SP
        GoAssemblyKind::Subq(AsmOperand::Int(10000), AsmOperand::Register(Register::SP)),
        // MOVQ    BP, 16(SP)
        GoAssemblyKind::Movq(
            AsmOperand::Register(Register::BP),
            AsmOperand::RegisterWithOffset(RegisterWithOffset {
                offset: 16,
                register: Register::SP,
            }),
        ),
    ]);
    let body = GoAssembly(func.stmts.into_iter().fold(vec![], |mut body, stmt| {
        body.append(&mut compile_stmt(stmt).0);
        body
    }));

    let epilogue = GoAssembly(vec![
        GoAssemblyKind::Movq(
            AsmOperand::RegisterWithOffset(RegisterWithOffset {
                offset: 16,
                register: Register::SP,
            }),
            AsmOperand::Register(Register::BP),
        ),
        GoAssemblyKind::Addq(AsmOperand::Int(10000), AsmOperand::Register(Register::SP)),
    ]);
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
        Instruction::Ret(typ, a) => vec![GoAssemblyKind::Ret],
        Instruction::Call(Operand::Var(var)) => {
            let call_op = AsmOperand::Ident(var);
            vec![
                GoAssemblyKind::Call(call_op),
                GoAssemblyKind::Movq(
                    AsmOperand::Register(Register::AX),
                    AsmOperand::RegisterWithOffset(RegisterWithOffset {
                        offset: 8,
                        register: Register::SP,
                    }),
                ),
            ]
        }
    })
}
