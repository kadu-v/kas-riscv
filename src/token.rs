use std::collections::HashMap;
use once_cell::sync::Lazy;

static KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    [("lw", TokenKind::LW)].into_iter().cloned().collect::<HashMap<&str, TokenKind>>()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    NewLine, // '\n', '\r
    Comma,  // ","
    LParen, // "("
    RParen, // ")"
    Symbol(String), // "lw"
    Number(u16),
    EOF,
    ILEGAL,

    // オペコード
    LW,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
}

pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    if let Some(kind) = KEYWORDS.get(ident) {
        return Some(kind.clone())
    }
    None
}