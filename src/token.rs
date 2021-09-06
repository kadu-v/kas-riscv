use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    [
        ("lw", TokenKind::LW),
        ("sw", TokenKind::SW),
        ("addi", TokenKind::ADDI),
        ("add", TokenKind::ADD),
        ("sub", TokenKind::SUB),
        ("and", TokenKind::AND),
        ("or", TokenKind::OR),
        ("xor", TokenKind::XOR),
        ("andi", TokenKind::ANDI),
        ("ori", TokenKind::ORI),
        ("xori", TokenKind::XORI),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, TokenKind>>()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    NewLine,        // '\n', '\r
    Comma,          // ","
    LParen,         // "("
    RParen,         // ")"
    Symbol(String), // "lw"
    Number(usize),  // 123...
    EOF,
    ILEGAL,

    // オペコード
    // I形式
    LW,   // lw
    ADDI, // addi

    // S形式
    SW, // sw

    // R形式
    ADD,  // add
    SUB,  // sub
    AND,  // and
    OR,   // or
    XOR,  // xor
    ANDI, // andi
    ORI,  // ori
    XORI, // xori
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
}

// ident が予約後の場合は対応する TokenKind を返す
pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    if let Some(kind) = KEYWORDS.get(ident) {
        return Some(kind.clone());
    }
    None
}
