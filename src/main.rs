use std::{env, ops::Deref, process::exit};

trait Emit {
    fn emit(&self) -> String;
}

#[derive(Debug)]
pub struct FunctionBlock {
    name: String,
    block: Statements,
}

impl FunctionBlock {
    fn new(name: String, block: Statements) -> Self {
        Self { name: name, block }
    }
}

impl Emit for FunctionBlock {
    fn emit(&self) -> String {
        format!(
            r#"TEXT ·{name}(SB),$0
{block}
"#,
            name = self.name,
            block = self.block.emit()
        )
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
    Assign { lhs: Register, rhs: Expr },
}

impl Emit for Statement {
    fn emit(&self) -> String {
        match self {
            Statement::Ret(n) => format!(
                r#"    MOVL ${n}, ret+0(FP)
    RET
"#,
                n = n
            ),
            Statement::Assign { lhs, rhs }
                if lhs == &Register::AX && rhs.will_store_ax_register() =>
            {
                rhs.emit()
            }
            // Statement::Assign {lhs, rhs} => format!("")
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Register {
    AX,
}

impl Emit for Register {
    fn emit(&self) -> String {
        match self {
            &Register::AX => "AX",
        }
        .into()
    }
}

#[derive(Debug)]
pub enum Expr {
    Add(Box<Operand>, Box<Operand>),
}

impl Expr {
    fn will_store_ax_register(&self) -> bool {
        match self {
            Expr::Add(_, _) => true,
        }
    }
}

impl Emit for Expr {
    fn emit(&self) -> String {
        match self {
            Expr::Add(l, r) => format!("ADDL {} {}", l.emit(), r.emit()),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Literal(usize),
}

impl Emit for Operand {
    fn emit(&self) -> String {
        match self {
            Operand::Register(r) => r.emit(),
            Operand::Literal(lit) => format!("${}", lit),
        }
    }
}

fn main() {
    let input: Vec<_> = env::args().collect();
    if input.len() != 2 {
        eprintln!("usage: ggvm <expr>");
        exit(1)
    }
    let x = input[1].parse::<usize>().unwrap();
    let ret_stmt = Statement::Ret(x);
    // let add1 = Statement::Assign {
    //     lhs: Register::AX,
    //     rhs: Expr::Add(Operand::Register(Register::AX), )
    // };
    let add2 = FunctionBlock::new("add2".to_string(), Statements(vec![add1, ret_stmt]));
    println!("{}", add2.emit());
}
