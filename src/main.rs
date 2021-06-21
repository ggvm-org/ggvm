use std::{env, process::exit};

trait Emit {
    fn emit(&self) -> String;
}

#[derive(Debug)]
pub struct FunctionBlock {
    name: String,
    block: Statements
}

impl FunctionBlock {
    fn new(name: String, block: Statements) -> Self {
        Self{name: name, block}
    }
}

impl Emit for FunctionBlock {
    fn emit(&self) -> String {
        format!(r#"TEXT ·{name}(SB),$0
{block}
"#, name=self.name, block=self.block.emit())
    }
}

#[derive(Debug)]
pub struct Statements(Vec<Statement>);

impl Deref for Statements {
    type Target = Vec<Statement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Emit for Statements {
    fn emit(&self) -> String {
        self.iter()
            .map(|stmt| stmt.emit())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
pub enum Statement {
    Ret(usize),
}

impl Emit for Statement {
    fn emit(&self) -> String {
        match self {
            Statement::Ret(n) => format!(
r#"    MOVL ${n}, ret+0(FP)
    RET
"#, n=n),
        }
    }
}

fn main() {
    let input:Vec<_> = env::args().collect();
    if input.len() != 2 {
        eprintln!("usage: ggvm <expr>");
        exit(1)
    }
    let x = input[1].parse::<usize>().unwrap();
    let ret_stmt  =Statement::Ret(x);
    let add2  = FunctionBlock::new("add2".to_string(), Statements(vec![ret_stmt]));
    println!("{}", add2.emit());
}
