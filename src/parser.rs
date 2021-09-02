use std::result::Result;

use crate::lexer::*;
use crate::token::*;
use crate::token::TokenKind::*;
use crate::instructions::*;
use crate::instructions::InstType::*;

#[derive(Debug)]
pub struct Parser<'a> {
    l : &'a mut Lexer<'a>,
    cur_tok : Token,
    next_tok : Token,
}


impl<'a> Parser<'a> {
    // Parserのコンストラクター
    pub fn new(l: &'a mut Lexer<'a>) -> Self {
        let cur_tok = l.next_token();
        let next_tok = l.next_token();
        Parser { l, cur_tok, next_tok }
    }

    // 次のtokenをセットするメソッド
    fn next_token(&mut self) {
        self.cur_tok = self.next_tok.clone();
        self.next_tok = self.l.next_token();
    }

    // 命令列のパース
    pub fn parse(&mut self) -> Result<Inst, String> {
        match &self.cur_tok.kind {
            // 命令列の末尾を表す
            EOF => Ok(Inst { ty : EOINST }),
            // I 形式の命令
            LW => self.parse_lw(),
            _ => Err("unsupported instruction!!".to_string())
        }
    }

    // lw命令をparseするメソッド
    fn parse_lw(&mut self) -> Result<Inst, String> {
        let mut rd = 0;
        let mut offset = 0;
        let mut rs1 = 0;

        // 先頭はLWだとわかっているので、つぎのTokenに進める
        self.next_token();

        //　次のtokenはレジスタ番号を表す数字 "rd"
        // todo!("support symbol, zero, ra, ...");
        match &self.cur_tok.kind {
            Number(x) => { 
                rd = *x as u8;
                self.next_token();
            },
            k => { 
                return Err(format!("expected number, but got {:?}", k)); 
            }
        };

        // 次のtokenは Comma
        match &self.cur_tok.kind {
            Comma => self.next_token(),
            k => {
                return Err(format!("expected \",\", but got {:?}", k))
            }
        };

        // 次のtokenは Number(x)
        match &self.cur_tok.kind {
            Number(x) => {
                offset = *x;
                self.next_token();
            }
            k => {
                return Err(format!("expected number, but got {:?}", k))
            }
        };

        // 次のtokenは LParen
        match &self.cur_tok.kind {
            LParen => self.next_token(),
            k => {
                return Err(format!("expected \"(\", but got {:?}", k))
            }
        };

        //　次のtokenは Number(x)
        match &self.cur_tok.kind {
            Number(x) => {
                rs1 = *x as u8;
                self.next_token();
            }
            k => {
                return Err(format!("expected number, but got {:?}", k))
            }
        };

        // 次のtokenは RParen
        match &self.cur_tok.kind {
            RParen => self.next_token(),
            k => {
                return Err(format!("expected \")\", bot got {:?}", k))
            }
        };

        // 命令列の末端は改行文字 または EOF
        match &self.cur_tok.kind {
            EOF => (),
            NewLine => self.next_token(),
            k => {
                return Err(format!("expected \"\\n\" or \"\\r\", bot got {:?}", k))
            }
        };

        Ok(Inst { ty : I { imm: offset, rs1: rs1, funct3: 0b010, rd: rd, opcode: 0b0000011 }})
    }
}


#[cfg(test)]
mod parser_tests {
    use crate::token::TokenKind::*;
    use crate::lexer::*;
    use crate::parser::*;


    #[test]
    fn test_next_token() {
        let s: &str = "lw 6, 16(10)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        
        assert_eq!(p.cur_tok.kind, LW);
        assert_eq!(p.next_tok.kind, Number(6));

        p.next_token();
        assert_eq!(p.cur_tok.kind, Number(6));
        assert_eq!(p.next_tok.kind, Comma);
    }
}