use std::fmt;

pub struct Func {
    name: String,
    ret_type: Type,
    wrap: &'static str,
}

impl Func {
    pub fn new(name: String, ret_type: Type, wrap: &'static str) -> Self {
        Self {
            name,
            ret_type,
            wrap,
        }
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let ret_type = &self.ret_type;
        let wrap = self.wrap;
        write!(f, "func {name}(x int) {ret_type}{{ return {wrap}(x) }}")
    }
}

pub struct Arg(String, Type);

pub enum Type {
    Int,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Int => "int",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_display_snapshot;
    #[test]
    fn insta_rand_intn() {
        let func = Func::new("randIntn".to_string(), Type::Int, "rand.Intn");
        assert_display_snapshot!(func);
    }
}
