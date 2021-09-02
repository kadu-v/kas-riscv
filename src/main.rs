use kas_riscv::lexer::Lexer;
use kas_riscv::parser::Parser;
use kas_riscv::code_gen::gen_hex;
fn main() {
    let x: &str = "lw 6, 8(0)\n";
    let mut l = Lexer::new(x);
    // for tok in l.into_iter() {
    //     println!("{:?}", tok);
    // }

    let mut p = Parser::new(&mut l);
    let inst = p.parse().unwrap();
    let code = gen_hex(&inst);
    println!("{}", inst);
    println!("{}", code);
}
