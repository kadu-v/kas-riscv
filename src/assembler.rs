use crate::code_gen::{gen_bin, gen_hex};
use crate::instructions::{Inst, InstType::*};
use crate::lexer::*;
use crate::parser::*;

// 文字列を入力されて、文字列を一行ずつ 字句解析 -> 構文解析 -> コード生成 をしてすべての文字列を結合する
// 字句解析 -> 構文解析 -> コード生成 -> 文字列の結合
pub fn assemble_hex(input: &str) -> std::result::Result<String, String> {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let mut src = "".to_string();

    while let inst = p.parse()? {
        match inst {
            Inst { ty: EOINST } => return Ok(src),
            i => {
                let c = gen_hex(&i);
                src = format!("{}{}", src, c);
            }
        }
    }
    Err("unreachable in assemble_hex !!".to_string())
}

// 文字列を入力されて、文字列を一行ずつ 字句解析 -> 構文解析 -> コード生成 をしてすべての文字列を結合する
// 字句解析 -> 構文解析 -> コード生成 -> 文字列の結合
pub fn assemble_bin(input: &str) -> std::result::Result<String, String> {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let mut src = "".to_string();

    while let inst = p.parse()? {
        match inst {
            Inst { ty: EOINST } => return Ok(src),
            i => {
                let c = gen_bin(&i);
                src = format!("{}{}\n", src, c);
            }
        }
    }
    Err("unreachable in assemble_hex !!".to_string())
}
