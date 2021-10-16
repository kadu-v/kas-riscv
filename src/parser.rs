use std::result::Result;

use crate::asm::{self, Asm, AsmKind};

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
            SLTI => self.parse_i_slti(),
            SLTIU => self.parse_i_sltiu(),
            SLLI => self.parse_i_slli(),
            SRLI => self.parse_i_srli(),
            SRAI => self.parse_i_srai(),
            // // S形式の命令
            SW => self.parse_s_sw(),
            // // R形式の命令
            ADD => self.parse_r_add(),
            SUB => self.parse_r_sub(),
            AND => self.parse_r_and(),
            OR => self.parse_r_or(),
            XOR => self.parse_r_xor(),
            SLT => self.parse_r_slt(),
            SLTU => self.parse_r_sltu(),
            SLL => self.parse_r_sll(),
            SRL => self.parse_r_srl(),
            SRA => self.parse_r_sra(),
            // B形式
            BEQ => self.parse_b_beq(),
            BNE => self.parse_b_bne(),
            BLT => self.parse_b_blt(),
            BGE => self.parse_b_bge(),
            BLTU => self.parse_b_bltu(),
            BGEU => self.parse_b_bgeu(),
            _ => Err("Parser::parse: unsupported instruction!!".to_string()),
        }
    }
    // cur_tok が kind と一致しているかチェックする
    fn read_token_kind(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.cur_tok.kind != kind {
            return Err(format!(
                "Parser::read_token_kind: expected {:?}, but got {:?}",
                kind, self.cur_tok.kind
            ));
        }
        self.next_token();
        Ok(())
    }

    // Number(x) トークンかをチェックし、数字を返す
    fn read_number_token(&mut self) -> Result<isize, String> {
        match self.cur_tok.kind {
            Number(x) => {
                self.next_token();
                Ok(x)
            }
            _ => Err(format!(
                "Parser::read_number_token: expected number, but got {:?}",
                self.cur_tok.kind
            )),
        }
    }

    fn read_number_or_symbol_token(&mut self) -> Result<(Option<isize>, Option<String>), String> {
        let asm_kind = match self.cur_tok.kind.clone() {
            Number(x) => Ok((Some(x), None)),
            Symbol(s) => Ok((None, Some(s))),
            _ => Err(format!(
                "Parser::read_number_orsymbol_token: expected number or symbol, but got {:?}",
                self.cur_tok.kind
            )),
        };
        self.next_token();
        asm_kind
    }

    // lw 命令をparseするメソッド
    fn parse_i_lw(&mut self) -> Result<Asm, String> {
        // 先頭はLWだとわかっているので、つぎのTokenに進める
        self.next_token();

        //　次のtokenはレジスタ番号を表す数字 "rd"
        let rd = self.read_number_token()?;

        // 次のtokenは Comma
        self.read_token_kind(Comma)?;

        // 次のtokenは Number(x)
        let imm = self.read_number_token()?;

        // 次のtokenは LParen
        self.read_token_kind(LParen)?;

        //　次のtokenは Number(x)
        let rs1 = self.read_number_token()?;

        // 次のtokenは RParen
        self.read_token_kind(RParen)?;

        // 命令列の末端は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::LW { imm, rs1, rd },
        })
    }

    // sw 命令を parse するメソッド
    fn parse_s_sw(&mut self) -> Result<Asm, String> {
        // 先頭はSWだとわかっているので、つぎのTokenに進める
        self.next_token();

        // 次のtokenは Number(x)
        let rs2 = self.read_number_token()?;

        // 次のtokenは Comma
        self.read_token_kind(Comma)?;

        // 次のtokenは Number(x)
        let imm = self.read_number_token()?;

        // 次のtokenは LParen
        self.read_token_kind(LParen)?;

        // 次のtokenは Number(x)
        let rs1 = self.read_number_token()?;

        // 次のtokenは RParen
        self.read_token_kind(RParen)?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SW { imm, rs2, rs1 },
        })
    }

    // addi 命令を parse するメソッド
    fn parse_i_addi(&mut self) -> Result<Asm, String> {
        // 先頭は ADDI だとわかっているので、次の token に進める
        self.next_token();

        // 次の token はNumber(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::ADDI { imm, rs1, rd },
        })
    }

    // add 命令を parse するメソッド
    fn parse_r_add(&mut self) -> Result<Asm, String> {
        // 先頭は ADD だとわかっているので、次の token をに進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::ADD { rs2, rs1, rd },
        })
    }

    fn parse_r_sub(&mut self) -> Result<Asm, String> {
        // 先頭は SUB だとわかっているので、次の token をに進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SUB { rs2, rs1, rd },
        })
    }

    fn parse_r_and(&mut self) -> Result<Asm, String> {
        // 先頭は AND だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::AND { rs2, rs1, rd },
        })
    }

    fn parse_r_or(&mut self) -> Result<Asm, String> {
        // 先頭は AND だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::OR { rs2, rs1, rd },
        })
    }

    fn parse_r_xor(&mut self) -> Result<Asm, String> {
        // 先頭は AND だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::XOR { rs2, rs1, rd },
        })
    }

    fn parse_r_slt(&mut self) -> Result<Asm, String> {
        // 先頭は SLT だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SLT { rs2, rs1, rd },
        })
    }

    fn parse_r_sltu(&mut self) -> Result<Asm, String> {
        // 先頭は SLTU だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SLTU { rs2, rs1, rd },
        })
    }

    fn parse_i_slti(&mut self) -> Result<Asm, String> {
        // 先頭は SLTI だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SLTI { imm, rs1, rd },
        })
    }

    fn parse_i_sltiu(&mut self) -> Result<Asm, String> {
        // 先頭は SLTIU だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SLTIU { imm, rs1, rd },
        })
    }

    fn parse_r_sll(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SLL { rs2, rs1, rd },
        })
    }

    fn parse_r_srl(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SRL { rs2, rs1, rd },
        })
    }

    fn parse_r_sra(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs2 = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SRA { rs2, rs1, rd },
        })
    }

    fn parse_i_slli(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SLLI { imm, rs1, rd },
        })
    }

    fn parse_i_srli(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SRLI { imm, rs1, rd },
        })
    }

    fn parse_i_srai(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let imm = self.read_number_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::SRAI { imm, rs1, rd },
        })
    }

    fn parse_b_beq(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)またはSymbol(s)
        let (imm, label) = self.read_number_or_symbol_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::BEQ {
                imm,
                rs1,
                rd,
                label,
            },
        })
    }

    fn parse_b_bne(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)またはSymbol(s)
        let (imm, label) = self.read_number_or_symbol_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::BNE {
                imm,
                rs1,
                rd,
                label,
            },
        })
    }

    fn parse_b_blt(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)またはSymbol(s)
        let (imm, label) = self.read_number_or_symbol_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::BLT {
                imm,
                rs1,
                rd,
                label,
            },
        })
    }

    fn parse_b_bge(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)またはSymbol(s)
        let (imm, label) = self.read_number_or_symbol_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::BGE {
                imm,
                rs1,
                rd,
                label,
            },
        })
    }

    fn parse_b_bltu(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)またはSymbol(s)
        let (imm, label) = self.read_number_or_symbol_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::BLTU {
                imm,
                rs1,
                rd,
                label,
            },
        })
    }

    fn parse_b_bgeu(&mut self) -> Result<Asm, String> {
        // 先頭は SLL だとわかっているので、次の token に進める
        self.next_token();

        // 次の token は Number(x)
        let rd = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)
        let rs1 = self.read_number_token()?;

        // 次の token は Comma
        self.read_token_kind(Comma)?;

        // 次の token は Number(x)またはSymbol(s)
        let (imm, label) = self.read_number_or_symbol_token()?;

        // 命令列の最後は改行文字
        self.read_token_kind(NewLine)?;

        Ok(Asm {
            kind: AsmKind::BGEU {
                imm,
                rs1,
                rd,
                label,
            },
        })
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::inst::{Inst, InstType::*};
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
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::ADD {
            rs2: 5,
            rs1: 10,
            rd: 0,
        };

        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_sub() {
        let s: &str = "sub 1, 11, 6\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;

        let expect = AsmKind::SUB {
            rs2: 6,
            rs1: 11,
            rd: 1,
        };

        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_and() {
        let s: &str = "and 31, 10, 1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::AND {
            rs2: 1,
            rs1: 10,
            rd: 31,
        };

        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_or() {
        let s: &str = "or 0, 100, 521\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::OR {
            rs2: 521,
            rs1: 100,
            rd: 0,
        };
        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_xor() {
        let s: &str = "xor 24, 111, 666\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::XOR {
            rs2: 666,
            rs1: 111,
            rd: 24,
        };
        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_slt() {
        let s: &str = "slt 24, 11, 5\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SLT {
            rs2: 5,
            rs1: 11,
            rd: 24,
        };
        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_r_sltu() {
        let s: &str = "sltu 24, 0, 10\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SLTU {
            rs2: 10,
            rs1: 0,
            rd: 24,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_i_slti() {
        let s: &str = "slti 3, 2, -1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SLTI {
            imm: -1,
            rs1: 2,
            rd: 3,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_i_sltiu() {
        let s: &str = "sltiu 24, 9, 666\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SLTIU {
            imm: 666,
            rs1: 9,
            rd: 24,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_i_sll() {
        let s: &str = "sll 24, 9, 3\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SLL {
            rs2: 3,
            rs1: 9,
            rd: 24,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_i_srl() {
        let s: &str = "srl 4, 9, 6\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SRL {
            rs2: 6,
            rs1: 9,
            rd: 4,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_i_sra() {
        let s: &str = "sra 5, 9, 16\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SRA {
            rs2: 16,
            rs1: 9,
            rd: 5,
        };
        assert_eq!(asm_kind, expect);
    }

    #[test]
    fn test_parser_i_slli() {
        let s: &str = "slli 5, 9, -16\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SLLI {
            imm: -16,
            rs1: 9,
            rd: 5,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_i_slai() {
        let s: &str = "srai 5, 9, -1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::SRAI {
            imm: -1,
            rs1: 9,
            rd: 5,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_b_beq() {
        let s: &str = "beq 20, 12, 11\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::BEQ {
            imm: Some(11),
            rs1: 12,
            rd: 20,
            label: None,
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_b_bne() {
        let s: &str = "bne 20, 12, A1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::BNE {
            imm: None,
            rs1: 12,
            rd: 20,
            label: Some("A1".to_string()),
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_b_blt() {
        let s: &str = "blt 0, 1, B1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::BLT {
            imm: None,
            rs1: 1,
            rd: 0,
            label: Some("B1".to_string()),
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_b_bge() {
        let s: &str = "bge 17, 16, VVV1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::BGE {
            imm: None,
            rs1: 16,
            rd: 17,
            label: Some("VVV1".to_string()),
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_b_bltu() {
        let s: &str = "bltu 7, 6, loop1\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::BLTU {
            imm: None,
            rs1: 6,
            rd: 7,
            label: Some("loop1".to_string()),
        };
        assert_eq!(asm_kind, expect);
    }
    #[test]
    fn test_parser_b_bgeu() {
        let s: &str = "bgeu 23, 4, loop2\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let asm_kind = p.parse().unwrap().kind;
        let expect = AsmKind::BGEU {
            imm: None,
            rs1: 4,
            rd: 23,
            label: Some("loop2".to_string()),
        };
        assert_eq!(asm_kind, expect);
    }
}
