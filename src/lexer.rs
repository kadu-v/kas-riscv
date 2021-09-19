use crate::token::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    const EOF_CONST: u8 = 0;
    // Lexer のコンストラクター
    pub fn new(input: &'a str) -> Self {
        let input = input.as_bytes();
        let mut l = Self {
            input: input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    // 次のトークンを返すメソッド
    // 現在の文字を検査して、次の文字をせっとしてから返す
    pub fn next_token(&mut self) -> Token {
        let mut tok = Token {
            kind: TokenKind::EOF,
        };

        // 空白を読み飛ばす
        self.skip_whitespaces();

        // Tokenを取り出す
        match self.ch {
            b'\r' | b'\n' => tok.kind = TokenKind::NewLine,
            b',' => tok.kind = TokenKind::Comma,
            b'(' => tok.kind = TokenKind::LParen,
            b')' => tok.kind = TokenKind::RParen,
            Self::EOF_CONST => tok.kind = TokenKind::EOF,
            _ => {
                if self.is_digit() || self.is_minus_lit() {
                    let s = String::from_utf8(self.read_number().to_vec()).unwrap();
                    println!("{}", "xxxxxxxxxxxxxxx");
                    let n = s.parse::<isize>().unwrap();
                    tok.kind = TokenKind::Number(n);
                    return tok;
                } else if self.is_letter() {
                    let ident = String::from_utf8(self.read_identifier().to_vec()).unwrap();
                    if let Some(kind) = lookup_keyword(&ident) {
                        tok.kind = kind;
                    } else {
                        tok.kind = TokenKind::Symbol(ident);
                    }
                    return tok;
                } else {
                    tok.kind = TokenKind::ILEGAL
                }
            }
        }
        self.read_char();
        tok
    }

    // 次の文字を読み込む
    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = Self::EOF_CONST; // EOFを表す定数
        } else {
            self.ch = self.input[self.next_pos];
        }

        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    // 次の文字を先読み
    fn peek_char(&self) -> Option<u8> {
        if self.next_pos >= self.input.len() {
            return None;
        }

        Some(self.input[self.next_pos])
    }

    //

    // 空白を読み飛ばす
    fn skip_whitespaces(&mut self) {
        while self.is_whitespace() {
            self.read_char()
        }
    }

    // Identifierを読み取るメソッド
    fn read_identifier(&mut self) -> &[u8] {
        let pos = self.pos;
        while self.is_letter() {
            self.read_char();
        }

        &self.input[pos..self.pos]
    }

    // 数字を読み取るメソッド
    // 負の数字にも対応
    pub fn read_number(&mut self) -> &[u8] {
        let pos = self.pos;
        // 先頭が '-' の時は一文字読み飛ばす
        if self.is_minus_lit() {
            self.read_char();
        }

        while self.is_digit() {
            self.read_char()
        }

        &self.input[pos..self.pos]
    }

    // 空白を判定するメソッド
    fn is_whitespace(&self) -> bool {
        self.ch as char == ' ' || self.ch as char == '\t'
    }

    // 改行を判定するメソッド
    fn is_newline(&self) -> bool {
        self.ch as char == '\r' || self.ch as char == '\n'
    }

    // 文字を判定するメソッド
    fn is_letter(&self) -> bool {
        (b'a' <= self.ch && self.ch <= b'z') || (b'A' <= self.ch && self.ch <= b'B')
    }

    // 数字を判定するメソッド
    fn is_digit(&self) -> bool {
        b'0' <= self.ch && self.ch <= b'9'
    }

    // '-' を判定するメソッド
    fn is_minus_lit(&mut self) -> bool {
        self.ch == b'-'
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    // EOF の代わりにNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.input.len() {
            return None;
        }
        Some(self.next_token())
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenKind},
    };

    #[test]
    fn test_read_char() {
        let s = "lw 6, 10(5)";
        let mut l = Lexer::new(s);

        assert_eq!(l.ch as char, 'l');
        l.read_char();
        assert_eq!(l.ch as char, 'w');
    }

    #[test]
    fn test_peek_char() {
        let s = "lw 6, 10(5)";
        let l = Lexer::new(s);

        assert_eq!(l.ch as char, 'l');
        assert_eq!(l.peek_char().unwrap() as char, 'w');
    }

    #[test]
    fn test_skip_whitespaces() {
        let s = "lw 6, 10(5)";
        let mut l = Lexer::new(s);
        l.read_char();
        l.read_char();
        l.skip_whitespaces();
        assert_eq!(l.ch as char, '6')
    }

    #[test]
    fn test_read_identifier() {
        let s = "lw 6, 10(5)";
        let mut l = Lexer::new(s);
        assert_eq!(l.read_identifier(), b"lw");
    }

    #[test]
    fn test_read_number1() {
        let s = "6888, 10(5)";
        let mut l = Lexer::new(s);
        assert_eq!(l.read_number(), b"6888");
    }

    #[test]
    fn test_read_number2() {
        let s = "-8";
        let mut l = Lexer::new(s);
        assert_eq!(l.read_number(), b"-8");
    }

    #[test]
    fn test_lexer_i_lw() {
        let s = "lw 6, 10(5)\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::LW
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(6)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(10)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::LParen
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(5)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::RParen
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_s_sw() {
        let s = "sw 100, 12(0)\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::SW
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(100)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(12)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::LParen
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(0)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::RParen
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_i_addi() {
        let s = "addi 100, 101, 20\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::ADDI
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(100)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(101)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(20)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }
    #[test]
    fn test_lexer_r_add() {
        let s = "add 32, 10, 5\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::ADD
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(32)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(10)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(5)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_r_sub() {
        let s = "sub 4, 11, 6\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::SUB
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(4)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(11)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(6)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_r_and() {
        let s = "and 9, 0, 7\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::AND
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(9)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(0)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(7)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_r_or() {
        let s = "or 31, 8, 100\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::OR
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(31)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(8)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(100)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_r_xor() {
        let s = "xor 5, 121, 521\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::XOR
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(5)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(121)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(521)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }

    #[test]
    fn test_lexer_r_andi() {
        let s = "andi 11, 123, 456\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::ANDI
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(11)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(123)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(456)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }
    #[test]
    fn test_lexer_r_ori() {
        let s = "ori 7, 321, 5521\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::ORI
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(7)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(321)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(5521)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }
    #[test]
    fn test_lexer_r_xori() {
        let s = "xori 5, 7, 13\n";
        let mut l = Lexer::new(s);
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::XORI
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(5)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(7)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Comma
            }
        );

        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::Number(13)
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::NewLine
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                kind: TokenKind::EOF
            }
        );
    }
}
