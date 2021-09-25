use std::result::Result;

use crate::asm::{Asm, AsmKind};

use crate::lexer::*;
use crate::token::TokenKind::*;
use crate::token::*;

#[derive(Debug)]
pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    cur_tok: Token,
    next_tok: Token,
}

impl<'a> Parser<'a> {
    // Parserのコンストラクター
    pub fn new(l: &'a mut Lexer<'a>) -> Self {
        let cur_tok = l.next_token();
        let next_tok = l.next_token();
        Parser {
            l,
            cur_tok,
            next_tok,
        }
    }

    // 次のtokenをセットするメソッド
    fn next_token(&mut self) {
        self.cur_tok = self.next_tok.clone();
        self.next_tok = self.l.next_token();
    }

    // 命令列のパース
    pub fn parse(&mut self) -> Result<Asm, String> {
        match &self.cur_tok.kind {
            // 命令列の末尾を表す
            EOF => Ok(Asm {
                kind: AsmKind::EOASM,
            }),
            // I 形式の命令
            LW => self.parse_i_lw(),
            ADDI => self.parse_i_addi(),
            // // S形式の命令
            SW => self.parse_s_sw(),
            // // R形式の命令
            ADD => self.parse_r_add(),
            // SUB => self.parse_r_sub(),
            // AND => self.parse_r_and(),
            // OR => self.parse_r_or(),
            // XOR => self.parse_r_xor(),
            _ => Err("unsupported instruction!!".to_string()),
        }
    }
    // cur_tok が kind と一致しているかチェックする
    fn check_token_kind(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.cur_tok.kind != kind {
            return Err(format!(
                "expected {:?}, but got {:?}",
                kind, self.cur_tok.kind
            ));
        }
        self.next_token();
        Ok(())
    }

    // Number(x) トークンかをチェックし、数字を返す
    fn check_number_token(&mut self) -> Result<isize, String> {
        match self.cur_tok.kind {
            Number(x) => {
                self.next_token();
                Ok(x)
            }
            _ => Err(format!("expected number, but got {:?}", self.cur_tok.kind)),
        }
    }

    // lw 命令をparseするメソッド
    fn parse_i_lw(&mut self) -> Result<Asm, String> {
        // 先頭はLWだとわかっているので、つぎのTokenに進める
        self.next_token();

        //　次のtokenはレジスタ番号を表す数字 "rd"
        let rd = self.check_number_token()?;

        // 次のtokenは Comma
        self.check_token_kind(Comma)?;

        // 次のtokenは Number(x)
        let imm = self.check_number_token()?;

        // 次のtokenは LParen
        self.check_token_kind(LParen)?;

        //　次のtokenは Number(x)
        let rs1 = self.check_number_token()?;

        // 次のtokenは RParen
        self.check_token_kind(RParen)?;

        // 命令列の末端は改行文字
        self.check_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::LW { imm, rs1, rd },
        })
    }

    // sw 命令を parse するメソッド
    fn parse_s_sw(&mut self) -> Result<Asm, String> {
        // 先頭はSWだとわかっているので、つぎのTokenに進める
        self.next_token();

        // 次のtokenは Number(x)
        let rs2 = self.check_number_token()?;

        // 次のtokenは Comma
        self.check_token_kind(Comma)?;

        // 次のtokenは Number(x)
        let imm = self.check_number_token()?;

        // 次のtokenは LParen
        self.check_token_kind(LParen)?;

        // 次のtokenは Number(x)
        let rs1 = self.check_number_token()?;

        // 次のtokenは RParen
        self.check_token_kind(RParen)?;

        // 命令列の最後は改行文字
        self.check_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SW { imm, rs2, rs1 },
        })
    }

    // addi 命令を parse するメソッド
    fn parse_i_addi(&mut self) -> Result<Asm, String> {
        // 先頭は ADDI だとわかっているので、次の token に進める
        self.next_token();

        // 次の token はNumber(x)
        let rd = self.check_number_token()?;

        // 次の token は Comma
        self.check_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.check_number_token()?;

        // 次の token は Comma
        self.check_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.check_number_token()?;

        // 命令列の最後は改行文字
        self.check_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::ADDI { imm, rs1, rd },
        })
    }

    // add 命令を parse するメソッド
    fn parse_r_add(&mut self) -> Result<Asm, String> {
        // 先頭は ADD だとわかっているので、次の token をに進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.check_number_token()?;

        // 次の token は Comma
        self.check_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.check_number_token()?;

        // 次の token は Comma
        self.check_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.check_number_token()?;

        // 命令列の最後は改行文字
        self.check_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::ADD { rs2, rs1, rd },
        })
    }

    // fn parse_r_sub(&mut self) -> Result<Inst, String> {
    //     // 先頭は ADD だとわかっているので、次の token をに進める
    //     self.next_token();

    //     // 次の token は Number(x)
    //     let rd = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs1 = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs2 = self.check_number_token()?;

    //     // 命令列の最後は改行文字
    //     self.check_token_kind(NewLine)?;

    //     Ok(Inst {
    //         ty: R {
    //             funct7: 0b0100000,
    //             rs2: rs2,
    //             rs1: rs1,
    //             funct3: 0b000,
    //             rd: rd,
    //             opcode: 0b0110011,
    //         },
    //     })
    // }

    // fn parse_r_and(&mut self) -> Result<Inst, String> {
    //     // 先頭は AND だとわかっているので、次の token に進める
    //     self.next_token();

    //     // 次の token は Number(x)
    //     let rd = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs1 = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs2 = self.check_number_token()?;

    //     // 命令列の最後は改行文字
    //     self.check_token_kind(NewLine)?;

    //     Ok(Inst {
    //         ty: R {
    //             funct7: 0b0000000,
    //             rs2: rs2,
    //             rs1: rs1,
    //             funct3: 0b111,
    //             rd: rd,
    //             opcode: 0b0110011,
    //         },
    //     })
    // }

    // fn parse_r_or(&mut self) -> Result<Inst, String> {
    //     // 先頭は AND だとわかっているので、次の token に進める
    //     self.next_token();

    //     // 次の token は Number(x)
    //     let rd = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs1 = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs2 = self.check_number_token()?;

    //     // 命令列の最後は改行文字
    //     self.check_token_kind(NewLine)?;

    //     Ok(Inst {
    //         ty: R {
    //             funct7: 0b0000000,
    //             rs2: rs2,
    //             rs1: rs1,
    //             funct3: 0b110,
    //             rd: rd,
    //             opcode: 0b0110011,
    //         },
    //     })
    // }

    // fn parse_r_xor(&mut self) -> Result<Inst, String> {
    //     // 先頭は AND だとわかっているので、次の token に進める
    //     self.next_token();

    //     // 次の token は Number(x)
    //     let rd = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs1 = self.check_number_token()?;

    //     // 次の token は Comma
    //     self.check_token_kind(Comma)?;

    //     // 次の token は Number(x)
    //     let rs2 = self.check_number_token()?;

    //     // 命令列の最後は改行文字
    //     self.check_token_kind(NewLine)?;

    //     Ok(Inst {
    //         ty: R {
    //             funct7: 0b0000000,
    //             rs2: rs2,
    //             rs1: rs1,
    //             funct3: 0b100,
    //             rd: rd,
    //             opcode: 0b0110011,
    //         },
    //     })
    // }
}

#[cfg(test)]
mod parser_tests {
    use crate::instructions::{Inst, InstType::*};
    use crate::lexer::*;
    use crate::parser::*;
    use crate::token::TokenKind::*;

    // lw rd imm(rs1)
    #[test]
    fn test_parser_i_lw() {
        let s: &str = "lw 6, 16(10)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::LW {
            imm: 16,
            rs1: 10,
            rd: 6,
        };

        assert_eq!(asm_kind, expect);
    }

    // sw rs1 imm(rs2)
    #[test]
    fn test_parser_s_sw() {
        let s: &str = "sw 6, 2357(0)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SW {
            imm: 2357,
            rs1: 0,
            rs2: 6,
        };

        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_i_addi() {
        let s: &str = "addi 6, 16, 10\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::ADDI {
            imm: 10,
            rs1: 16,
            rd: 6,
        };

        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_add() {
        let s: &str = "add 0, 10, 5\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let inst = p.parse().unwrap().kind;
        let expect = AsmKind::ADD {
            rs2: 5,
            rs1: 10,
            rd: 0,
        };
        // let expect = R {
        //     funct7: 0b0000000,
        //     rs2: 5,
        //     rs1: 10,
        //     funct3: 0b000,
        //     rd: 0,
        //     opcode: 0b0110011,
        // };

        assert_eq!(inst, expect);
    }

    // #[test]
    // fn test_parser_r_sub() {
    //     let s: &str = "sub 1, 11, 6\n";
    //     let mut l = Lexer::new(s);
    //     let mut p = Parser::new(&mut l);
    //     let inst = p.parse().unwrap().ty;
    //     let expect = R {
    //         funct7: 0b0100000,
    //         rs2: 6,
    //         rs1: 11,
    //         funct3: 0,
    //         rd: 1,
    //         opcode: 0b0110011,
    //     };

    //     assert_eq!(inst, expect);
    // }

    // #[test]
    // fn test_parser_r_and() {
    //     let s: &str = "and 31, 10, 1\n";
    //     let mut l = Lexer::new(s);
    //     let mut p = Parser::new(&mut l);
    //     let inst = p.parse().unwrap().ty;
    //     let expect = R {
    //         funct7: 0b0000000,
    //         rs2: 1,
    //         rs1: 10,
    //         funct3: 0b111,
    //         rd: 31,
    //         opcode: 0b0110011,
    //     };

    //     assert_eq!(inst, expect);
    // }

    // #[test]
    // fn test_parser_r_or() {
    //     let s: &str = "or 0, 100, 521\n";
    //     let mut l = Lexer::new(s);
    //     let mut p = Parser::new(&mut l);
    //     let inst = p.parse().unwrap().ty;
    //     let expect = R {
    //         funct7: 0b0000000,
    //         rs2: 521,
    //         rs1: 100,
    //         funct3: 0b110,
    //         rd: 0,
    //         opcode: 0b0110011,
    //     };

    //     assert_eq!(inst, expect);
    // }

    // #[test]
    // fn test_parser_r_xor() {
    //     let s: &str = "xor 24, 111, 666\n";
    //     let mut l = Lexer::new(s);
    //     let mut p = Parser::new(&mut l);
    //     let inst = p.parse().unwrap().ty;
    //     let expect = R {
    //         funct7: 0b0000000,
    //         rs2: 666,
    //         rs1: 111,
    //         funct3: 0b100,
    //         rd: 24,
    //         opcode: 0b0110011,
    //     };

    //     assert_eq!(inst, expect);
    // }
}
