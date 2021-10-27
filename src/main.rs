use std::{collections::HashMap, hash::Hash};

// run {
// let v: int = 1;
// return v;
// }

//
// TEXT main·run(SB), $0-16
// MOVQ    $1, r0+16(SP)
// MOVQ    r0+16, r0+8(SP)
// RET

#[derive(Debug, PartialEq)]
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
struct AnalyzedFunctionBlock {
    fb: FunctionBlock,
    stack_size: usize,
    var_to_align: HashMap<String, usize>,
}

impl AnalyzedFunctionBlock {
    fn new(fb: FunctionBlock) -> Self {
        let (stack_size, var_to_align) = Self::analyze(&fb);

        Self {
            fb,
            stack_size,
            var_to_align,
        }
    }

    fn analyze(fb: &FunctionBlock) -> (usize, HashMap<String, usize>) {
        let mut stack_size = 0;
        let mut var_to_align = HashMap::new();
        for stmt in fb.instructions.iter() {
            match stmt {
                Statement::Let(var_name, ..) => {
                    var_to_align.insert(var_name.to_string(), stack_size);
                    stack_size += 8;
                }
                _ => {}
            }
        }

        // +8 is for BP
        (stack_size + 8, var_to_align)
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Let(String, Literal),
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
            Statement::Let("v".to_string(), Literal::Integer(1)),
            Statement::Return(Literal::Ident("v".to_string())),
        ],
    );
    println!(
        r#"
TEXT main·run(SB), $0-16
SUBQ    $16, SP
MOVQ    BP, 8(SP)
LEAQ    8(SP), BP
MOVQ    $1, k(SP)
MOVQ    $1, r0+24(SP)
MOVQ    8(SP), BP
ADDQ    $16, SP
RET"#
    );
    // dbg!(AnalyzedFunctionBlock::new(run_block));

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
