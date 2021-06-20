use std::{env, process::exit};


fn main() {
    let input:Vec<_> = env::args().collect();
    if input.len() != 2 {
        eprintln!("usage: ggvm <filename>");
        exit(1)
    }
    let x = input[1].parse::<usize>().unwrap();
    // https://blog.lufia.org/entry/2021/03/17/113000
    println!("{}", format!(r#"TEXT ·add2(SB),$0-12
    MOVL i+0(FP),AX    // 引数iをAXレジスタに
    ADDL ${x}, AX        // {x}を加算
    MOVL AX, ret+8(FP) // 計算結果を戻り値として返す
    RET"#, x=x));
}
