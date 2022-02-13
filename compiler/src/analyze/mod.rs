use std::{collections::HashMap, hash::Hash};

use crate::Operand;

pub(crate) struct Environment<'a>(HashMap<Operand<'a>, usize>);

// func $add(%x int, %y int) int {
//   %z = add int %x, %y
// 	 ret int %z;
// }

// ((add ))

pub(crate) struct AnalyzeResult<'a> {
    stacksize: usize,
    env: Environment<'a>,
}

impl From<

impl Environment<'_> {
    fn new() -> Self {
        Self(HashMap::new())
    }
}
