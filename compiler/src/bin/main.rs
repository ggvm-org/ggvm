use std::io::Read;
use std::process;

extern crate compiler;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ggvm <file>");
        process::exit(1);
    }
    let path = std::path::PathBuf::from(args[1].clone());
    let mut file = std::fs::File::open(path).expect("file doesn't exist");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("idk");
    let parse = compiler::parse();
    let result = parse(&input);
    assert!(result.is_ok());
    let (_rest, func) = result.unwrap();
    let analyze_result = compiler::analyze(func);
    dbg!(&analyze_result);
    let compile = compiler::compile();
    let go_asm = compile(analyze_result);
    dbg!(go_asm);
}
