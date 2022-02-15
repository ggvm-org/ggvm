use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use crate::{Func, Instruction, Operand, Statement};

pub(crate) struct Environment(HashMap<Operand, usize>);

impl Deref for Environment {
    type Target = HashMap<Operand, usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Environment {
    // type Target = HashMap<Operand, usize>;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// func $add(%x int, %y int) int {
//   %z = add int %x, %y
// 	 ret int %z;
// }

// ((add ))

pub(crate) struct AnalyzeResult {
    stacksize: usize,
    // TODO: Add offset
    env: Environment,
    func: Func,
}

impl AnalyzeResult {
    fn new(stacksize: usize, env: Environment, func: Func) -> Self {
        Self {
            stacksize,
            env,
            func,
        }
    }
}

pub(crate) fn analyze(func: Func) -> AnalyzeResult {
    let stacksize = 10000;
    // x -> 4
    // y -> 8,
    // the key is ident then returns alignment
    let mut env = Environment::new();

    // returns the set of idents
    let offset = func.stmts.iter().fold(0, |mut offset, stmt| {
        let v = analyze_stmt(stmt);
        v.into_iter().for_each(|ident_str| {
            if !env.contains_key(&ident_str) {
                env.insert(ident_str, offset);
                offset += 4;
            }
        });
        offset
    });

    AnalyzeResult {
        stacksize,
        env,
        func,
    }
}

pub(crate) fn analyze_stmt(stmt: &Statement) -> HashSet<Operand> {
    let mut h = HashSet::new();
    match &stmt {
        &Statement::Local(op, b) => h.extend(analyze_operand(&op)),
        &Statement::Inst(inst) => h.extend(analyze_inst(&inst)),
    };
    h
}

pub(crate) fn analyze_inst(inst: &Instruction) -> HashSet<Operand> {
    let mut h = HashSet::new();
    match &inst {
        &Instruction::Add(_, left, right) => {
            h.extend(analyze_operand(&left));
            h.extend(analyze_operand(&right));
        }
        &Instruction::Ret(typ, op) => h.extend(analyze_operand(&op)),
    }
    h
}

pub(crate) fn analyze_operand(op: &Operand) -> HashSet<Operand> {
    let mut h = HashSet::new();
    match &op {
        &Operand::Var(s) => h.insert(Operand::Var(s.to_string())),
    };
    h
}

impl Environment {
    fn new() -> Self {
        Self(HashMap::new())
    }
}
