use std::collections::HashMap;

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
        let mut stack_size = 8;
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
        (stack_size, var_to_align)
    }
}

impl AnalyzedFunctionBlock {
    fn to_string(&self) -> String {
        let text = format!("TEXT main·{}(SB), $0\n", self.fb.name);
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
        format!("MOVQ    $1, {}(SP)\n", self.stack_size + 8)
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
    println!("{}", AnalyzedFunctionBlock::new(run_block).to_string());
}
