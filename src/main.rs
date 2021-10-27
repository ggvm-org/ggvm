// run {
// v: int = 1;
// return v;
// }

//
// TEXT main·run(SB), $0-16
// MOVQ    $1, r0+16(SP)
// MOVQ    r0+16, r0+8(SP)
// RET

struct FunctionBlock {
    name: String,
    instructions: Vec<Statement>,
}

impl FunctionBlock {
    fn new(name: String, instructions: Vec<Statement>) -> Self {
        FunctionBlock { name, instructions }
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Assign(String, Literal),
    Return(Literal),
}

#[derive(Debug, PartialEq)]
enum Literal {
    Integer(i64),
    Ident(String),
}

fn main() {
    let run_block = FunctionBlock::new(
        "run".to_string(),
        vec![
            Statement::Assign("v".to_string(), Literal::Integer(1)),
            Statement::Return(Literal::Ident("v".to_string())),
        ],
    );
    println!(
        r#"
TEXT main·run(SB), $0-16
SUBQ    $16, SP
MOVQ    BP, 8(SP)
LEAQ    8(SP), BP
MOVQ    $0, r0+24(SP)
MOVQ    $1, k(SP)
MOVQ    $1, r0+24(SP)
MOVQ    8(SP), BP
ADDQ    $16, SP
RET"#
    );

    //     println!(
    //         r#"
    //         TEXT	main·run(SB), $16-0
    // MOVQ	(TLS), CX
    // CMPQ	SP, 16(CX)
    // JLS	67
    // SUBQ	$16, SP
    // MOVQ	BP, 8(SP)
    // LEAQ	8(SP), BP
    // CALL	runtime.printlock(SB)
    // MOVQ	$3, (SP)
    // CALL	runtime.printint(SB)
    // CALL	runtime.printnl(SB)
    // CALL	runtime.printunlock(SB)
    // MOVQ	8(SP), BP
    // ADDQ	$16, SP
    // RET
    // NOP
    // CALL	runtime.morestack_noctxt(SB)
    // JMP	0"#
    //     )
}
