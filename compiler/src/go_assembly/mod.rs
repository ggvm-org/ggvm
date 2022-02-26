use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[macro_use]
pub mod operand;
#[macro_use]
pub mod directive;
#[macro_use]
pub mod register_with_offset;

pub use directive::*;
pub use register::*;
pub use register_with_offset::*;

impl GoAssembly {
    fn new_goroutine_prologue() -> Self {
        Self(vec![
            CMPQ!(SP, 16=>R14),
            PCDATA!(0, -2),
            JLS!(@epi),
            directive!(@body),
        ])
    }

    fn new_goroutine_epilogue() -> Self {
        Self(vec![
            directive!(@epi),
            directive!(NOP),
            PCDATA!(1, 2),
            PCDATA!(0, -2),
            CALL!(runtime.morestack_noctxt),
            PCDATA!(0, -1),
            JMP!(@body),
        ])
    }
}
#[derive(Debug)]
pub struct GoAssembly(pub(crate) Vec<Directive>);

impl Deref for GoAssembly {
    type Target = Vec<Directive>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GoAssembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for GoAssembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stmts_str = self.iter().fold(String::new(), |asm_body_str, goasm| {
            format!("{}\n{}", asm_body_str, goasm.to_string())
        });
        write!(f, "{stmts_str}")
    }
}

#[cfg(test)]
mod insta {
    use crate::go_assembly::GoAssembly;
    use insta::assert_display_snapshot;

    macro_rules! insta_test {
        ($testname:ident: $($testcases:expr),+) => {
            #[test]
            fn $testname() {
                $(assert_display_snapshot!($testcases);)+
            }
        };
    }

    insta_test!(go_routine_prologue: GoAssembly::new_goroutine_prologue());

    insta_test!(go_routine_epilogue: GoAssembly::new_goroutine_epilogue());
}
