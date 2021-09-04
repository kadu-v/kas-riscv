use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    [
        ("lw", TokenKind::LW),
        ("sw", TokenKind::SW),
        ("addi", TokenKind::ADDI),
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
    Number(usize),
    EOF,
    ILEGAL,

    // オペコード
    // I形式
    LW,
    ADDI,
    // S形式
    SW,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
}

// identが予約後の場合は対応するTokenKindを返す
pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    if let Some(kind) = KEYWORDS.get(ident) {
        return Some(kind.clone());
    }
    None
}
