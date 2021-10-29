use std::collections::HashMap;

use p9_asm::{operand, popq, pushq, text};

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
        (stack_size, var_to_align)
    }
}

impl AnalyzedFunctionBlock {
    fn to_string(&self) -> String {
        let text = text!(run);
        let prologue = format!(
            "{}{}{}",
            format!("SUBQ ${}, SP\n", self.stack_size),
            format!("MOVQ BP, {}(SP)\n", self.stack_size - 8),
            format!("LEAQ {}(SP), BP\n", self.stack_size - 8)
        );
        let code = self.emit_fb();

        // let epilogue = format!("{}{}{}", "MOVQ BP, SP\n", popq!(BP), "RET\n");
        // TODO: write epilogue
        format!("{}{}{}", text, prologue, code)
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
            Statement::Let(var_name, lit) => {
                let expr_emitted = lit.emit(var_to_align);
                let popq = popq!(AX);
                if let Some(align) = var_to_align.get(var_name) {
                    let alloc = format!("MOVQ AX, r0+{}(SP)\n", align);
                    format!("{}{}{}", expr_emitted, popq, alloc)
                } else {
                    unreachable!()
                }
            }
            Statement::Return(lit) => {
                let emit_expr = lit.emit(var_to_align);
                let popq = popq!(AX);
                let alloc = format!("MOVQ AX, r0+{}(SP)\n", stack_size);
                let epilogue = format!(
                    "{}{}{}{}",
                    format!("MOVQ AX, ret+8(FP)\n"),
                    format!("MOVQ {}(SP), BP\n", stack_size - 8),
                    format!("ADDQ ${}, SP\n", stack_size),
                    format!("RET\n")
                );
                format!("{}{}{}{}", emit_expr, popq, alloc, epilogue)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Integer(i64),
    Ident(String),
}

impl Expression {
    fn emit(&self, var_to_align: &HashMap<String, usize>) -> String {
        match self {
            Self::Integer(n) => pushq!(n),
            Self::Ident(id) => {
                if let Some(align) = var_to_align.get(id) {
                    let align = format!("LEAQ {}+{}(SP), AX\n", id, align);
                    format!("{}{}", align, pushq!(AX))
                } else {
                    unreachable!()
                }
            }
        }
    }
}

fn main() {
    let run_block = FunctionBlock::new(
        "run".to_string(),
        vec![
            Statement::Let("z".to_string(), Expression::Integer(100)),
            Statement::Return(Expression::Ident("z".to_string())),
        ],
    );
    let afb = AnalyzedFunctionBlock::new(run_block);
    println!("#include \"go_asm.h\"\n");
    println!("{}", afb.to_string());
    dbg!(afb);
}
