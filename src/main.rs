// https://blog.lufia.org/entry/2021/03/17/113000
const ADD2: &str = r#"TEXT ·add2(SB),$0-12
MOVL i+0(FP),AX    // 引数iをAXレジスタに
ADDL $2, AX        // 2を加算
MOVL AX, ret+8(FP) // 計算結果を戻り値として返す
RET"#;

fn main() {
    println!("{}", ADD2);
}
