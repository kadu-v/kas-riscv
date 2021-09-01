use kas_riscv::lexer::Lexer;

fn main() {
    let x: &str = "lw 6, 16(10)\n";
    let lex = Lexer::new(x);
    for tok in lex {
        println!("{:?}", tok);
    }
}
