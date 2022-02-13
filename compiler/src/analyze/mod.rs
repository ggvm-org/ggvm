use std::{collections::HashMap, ops::Deref};

use crate::{Func, Operand};

pub(crate) struct Environment<'a>(HashMap<Operand<'a>, usize>);

impl<'a> Deref for Environment<'a> {
    type Target = HashMap<Operand<'a>, usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// func $add(%x int, %y int) int {
//   %z = add int %x, %y
// 	 ret int %z;
// }

// ((add ))

pub(crate) struct AnalyzeResult<'a> {
    stacksize: usize,
    env: Environment<'a>,
}

impl<'a> AnalyzeResult<'a> {
    fn new(stacksize: usize, env: Environment<'a>) -> Self {
        Self { stacksize, env }
    }
}

pub(crate) fn analyze(func: Func) -> AnalyzeResult {
    let stacksize = 10000;
    let mut env = Environment::new();

    AnalyzeResult { stacksize, env }
}

impl Environment<'_> {
    fn new() -> Self {
        Self(HashMap::new())
    }
}
