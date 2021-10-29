pub mod terminal;

use std::fmt::{self, Display, Formatter};
use std::u32;

use terminal::Terminal;

#[derive(Clone, Debug)]
pub struct Pos {
    pub byte: u64,
    pub column: u32,
    pub filename: String,
    pub length: usize,
    pub line: u32,
}

impl Pos {}

impl Pos {
    pub fn new(line: u32, column: u32, byte: u64, filename: &str, length: usize) -> Self {
        Pos {
            byte,
            column,
            filename: filename.to_string(),
            length,
            line,
        }
    }

    pub fn show(&self, terminal: &Terminal) {
        eprintln!(
            "   {}{}-->{}{} {}:{}:{}",
            terminal.bold(),
            terminal.blue(),
            terminal.reset_color(),
            terminal.end_bold(),
            self.filename,
            self.line,
            self.column
        )
    }
}

impl Display for Pos {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}:{}:", self.line, self.column)
    }
}
