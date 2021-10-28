use std::collections::HashMap;

use p9_asm::text;

// run {
// let v: int = 1;
// return v;
// }

// TEXT は通常 2 つ引数を取り, 第１引数に名前, 第 2 引数にサイズを指定できます. 第 1 引数の 名前には CALL 命令などの対象として利用するシンボルを指定します.
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

impl AnalyzedFunctionBlock {
    fn to_string(&self) -> String {
        let text = text!(run);
        let prologue = format!(
            r#"SUBQ    ${}, SP
MOVQ    BP, 8(SP)
LEAQ    8(SP), BP
"#,
            self.stack_size
        );

        let code = self.emit_fb();

        let epilogue = format!(
            r#"MOVQ    8(SP), BP
ADDQ    ${}, SP
RET
"#,
            self.stack_size
        );

        format!("{}{}{}{}", text, prologue, code, epilogue)
    }

    fn emit_fb(&self) -> String {
        self.fb
            .instructions
            .iter()
            .fold(String::new(), |code, stmt| {
                format!("{}{}", code, stmt.emit(self.stack_size, &self.var_to_align))
            })
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Let(String, Expression),
    Return(Expression),
}

impl Statement {
    fn emit(&self, stack_size: usize, var_to_align: &HashMap<String, usize>) -> String {
        match self {
            Statement::Let(var_name, lit) => Self::emit_let(var_name, lit, var_to_align),
            Statement::Return(lit) => match lit {
                Expression::Integer(n) => format!("MOVQ ${} r0+{}(SP)\n", n, stack_size + 8),
                Expression::Ident(i) => {
                    let src_align = var_to_align.get(i).unwrap();
                    format!(
                        r#"MOVQ {}(SP), AX
MOVQ AX, r0+{}(SP)
"#,
                        src_align,
                        stack_size + 8
                    )
                }
            },
        }
    }

    fn emit_let(var_name: &str, lit: &Expression, var_to_align: &HashMap<String, usize>) -> String {
        if let Some(align) = var_to_align.get(var_name) {
            match lit {
                Expression::Integer(n) => format!("MOVQ ${}, {}(SP)\n", n, align),
                Expression::Ident(i) => {
                    let src_align = var_to_align.get(i).unwrap();
                    format!(
                        r#"MOVQ {}(SP), AX
MOVQ AX, {}(SP)"#,
                        src_align, align
                    )
                }
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Integer(i64),
    Ident(String),
}

fn main() {
    let run_block = FunctionBlock::new(
        "run".to_string(),
        vec![
            Statement::Let("v".to_string(), Expression::Integer(200)),
            Statement::Let("x".to_string(), Expression::Integer(100)),
            Statement::Return(Expression::Ident("x".to_string())),
        ],
    );
    println!("{}", AnalyzedFunctionBlock::new(run_block).to_string());
}
