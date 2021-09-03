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
            // S形式の命令
            SW => self.parse_sw(),
            _ => Err("unsupported instruction!!".to_string())
        }
    }
    // cur_tokがkindと一致しているかチェックする
    fn check_token_kind(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.cur_tok.kind != kind {
            return Err(format!("expected {:?}, but got {:?}", kind, self.cur_tok.kind))
        }
        self.next_token();
        Ok(())
    }

    // Number(x) トークンかをチェックし、数字を返す
    fn check_number_token(&mut self) -> Result<usize, String> {
        match self.cur_tok.kind {
            Number(x) => {
                self.next_token();
                Ok(x)
            },
            _ => Err(format!("expected number, but got {:?}", self.cur_tok.kind))
        }
    }

    // lw命令をparseするメソッド
    fn parse_lw(&mut self) -> Result<Inst, String> {

        // 先頭はLWだとわかっているので、つぎのTokenに進める
        self.next_token();

        //　次のtokenはレジスタ番号を表す数字 "rd"
        // todo!("support symbol, zero, ra, ...");
        let rd = self.check_number_token()?;

        // 次のtokenは Comma
        self.check_token_kind(Comma)?;

        // 次のtokenは Number(x)
        let offset = self.check_number_token()?;

        // 次のtokenは LParen
        self.check_token_kind(LParen)?;

        //　次のtokenは Number(x)
        let rs1 = self.check_number_token()?;

        // 次のtokenは RParen
        self.check_token_kind(RParen)?;

        // 命令列の末端は改行文字
        self.check_token_kind(NewLine)?;

        Ok(Inst { ty : I { imm: offset, rs1: rs1, funct3: 0b010, rd: rd, opcode: 0b0000011 }})
    }

    fn parse_sw(&mut self) -> Result<Inst, String> {
        // 先頭はSWだとわかっているので、つぎのTokenに進める
        self.next_token();

        // 次のtokenは Number(x)
        let rs2 = self.check_number_token()?;

        // 次のtokenは Comma
        self.check_token_kind(Comma)?;

        // 次のtokenは Number(x)
        let offset = self.check_number_token()?;

        // 次のtokenは LParen
        self.check_token_kind(LParen)?;

        // 次のtokenは Number(x)
        let rs1 = self.check_number_token()?;

        // 次のtokenは RParen
        self.check_token_kind(RParen)?;

        // 命令列の最後は改行文字
        self.check_token_kind(NewLine)?;

        let imm_1 = offset >> 5;
        let imm_2 = offset & 0b11111;

        Ok(Inst { ty : S { imm_1: imm_1,  rs2: rs2, rs1: rs1, funct3: 0b010,  imm_2: imm_2, opcode: 0b0100011 }})
    }
}


#[cfg(test)]
mod parser_tests {
    use crate::token::TokenKind::*;
    use crate::lexer::*;
    use crate::parser::*;
    use crate::instructions::{Inst, InstType::*};


    #[test]
    fn test_parser_lw() {
        let s: &str = "lw 6, 16(10)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let inst = p.parse().unwrap().ty;
        let expect = I { imm: 16, rs1: 10, funct3: 2, rd: 6, opcode: 3 };

        assert_eq!(inst, expect);
    }


    #[test]
    fn test_parser_sw() {
        let s: &str = "sw 6, 2357(0)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let inst = p.parse().unwrap().ty;
        let expect = S { imm_1: 73, rs2: 6, rs1: 0,  funct3: 2, imm_2: 21, opcode: 35 };

        assert_eq!(inst, expect);
    }
}