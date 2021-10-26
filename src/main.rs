fn main() {
    println!(
        r#"
TEXT main·run(SB), $0-8
MOVQ    $1, r0+8(SP)
RET"#
    );
}
