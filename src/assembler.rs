use std::collections::HashMap;
use std::iter::Inspect;

use crate::asm::{Asm, AsmKind::*};
use crate::code_gen::{gen_bin, gen_hex};
use crate::instructions::{Inst, InstType::*};
use crate::parser::*;

pub struct Assembler<'a> {
    p: &'a mut Parser<'a>,
}

impl<'a> Assembler<'a> {
    // Assemblerのコンストラクター
    pub fn new(p: &'a mut Parser<'a>) -> Self {
        Self { p }
    }

    //
    pub fn assemble(&mut self) -> Result<Inst, String> {
        let asm = self.p.parse()?;
        let inst_type = match asm.kind {
            LW { imm, rs1, rd } => I {
                imm: imm,
                rs1: rs1,
                funct3: 0b010,
                rd: rd,
                opcode: 0b000011,
            },
            SW { imm, rs1, rs2 } => {
                let imm_1 = (0b111111100000 & imm) >> 5;
                let imm_2 = 0b000000011111 & imm;
                S {
                    imm_1: imm_1,
                    rs2: rs2,
                    funct3: 0b010,
                    rs1: rs1,
                    imm_2: imm_2,
                    opcode: 0b0100011,
                }
            }
            ADDI { imm, rs1, rd } => I {
                imm: imm,
                rs1: rs1,
                funct3: 0b000,
                rd: rd,
                opcode: 0b0010011,
            },
            ADD { rs2, rs1, rd } => R {
                funct7: 0b0000000,
                rs2: rs2,
                rs1: rs1,
                funct3: 0b000,
                rd: rd,
                opcode: 0b0110011,
            },
            SUB { rs2, rs1, rd } => R {
                funct7: 0b0100000,
                rs2: rs2,
                rs1: rs1,
                funct3: 0b000,
                rd: rd,
                opcode: 0b0110011,
            },
            EOASM => EOINST,
            x => return Err(format!("Assembler::assemble: {:?} is not implemnted", x)),
        };

        Ok(Inst { ty: inst_type })
    }

    // 全ての文字列をアセンブラに変換
    #[allow(irrefutable_let_patterns)]
    pub fn assemble_all(&mut self) -> Result<String, String> {
        let mut src = "".to_string();
        while let inst = self.assemble()? {
            match inst {
                Inst { ty: EOINST } => return Ok(src),
                i => {
                    let b = gen_bin(&i);
                    src = format!("{}{}\n", src, b);
                }
            }
        }
        Err("Assembler::assemble_all: unreachable !!".to_string())
    }

    //
    fn make_symbol_table(&mut self) -> HashMap<String, usize> {
        unimplemented!("Assemble::make_symbol_table: make_symbol_table have not implemented yet.")
    }
}

// // 文字列を入力されて、文字列を一行ずつ 字句解析 -> 構文解析 -> コード生成 をしてすべての文字列を結合する
// // 字句解析 -> 構文解析 -> コード生成 -> 文字列の結合
// #[allow(irrefutable_let_patterns)]
// pub fn assemble_hex(input: &str) -> std::result::Result<String, String> {
//     let mut l = Lexer::new(input);
//     let mut p = Parser::new(&mut l);
//     let mut src = "".to_string();

//     while let inst = p.parse()? {
//         match inst {
//             Inst { ty: EOINST } => return Ok(src),
//             i => {
//                 let c = gen_hex(&i);
//                 src = format!("{}{}", src, c);
//             }
//         }
//     }
//     Err("unreachable in assemble_hex !!".to_string())
// }

// // 文字列を入力されて、文字列を一行ずつ 字句解析 -> 構文解析 -> コード生成 をしてすべての文字列を結合する
// // 字句解析 -> 構文解析 -> コード生成 -> 文字列の結合
// #[allow(irrefutable_let_patterns)]
// pub fn assemble_bin(input: &str) -> std::result::Result<String, String> {
//     let mut l = Lexer::new(input);
//     let mut p = Parser::new(&mut l);
//     let mut src = "".to_string();

//     while let inst = p.parse()? {
//         match inst {
//             Inst { ty: EOINST } => return Ok(src),
//             i => {
//                 let c = gen_bin(&i);
//                 src = format!("{}{}\n", src, c);
//             }
//         }
//     }
//     Err("unreachable in assemble_hex !!".to_string())
// }

mod assemble_tests {
    use crate::asm::{Asm, AsmKind};
    use crate::assembler::Assembler;
    use crate::instructions::{Inst, InstType::*};
    use crate::lexer::*;
    use crate::parser::*;

    #[test]
    fn test_assembler_i_lw() {
        let s: &str = "lw 6, 16(10)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let mut a = Assembler::new(&mut p);
        let inst_ty = a.assemble().unwrap().ty;
        let expect = I {
            imm: 16,
            rs1: 10,
            funct3: 0b010,
            rd: 6,
            opcode: 0b0000011,
        };
        assert_eq!(inst_ty, expect);
    }

    #[test]
    fn test_assembler_s_sw() {
        let s: &str = "sw 6, 2357(0)\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let mut a = Assembler::new(&mut p);
        let inst_ty = a.assemble().unwrap().ty;
        let expect = S {
            imm_1: 73,
            rs2: 6,
            rs1: 0,
            funct3: 0b010,
            imm_2: 21,
            opcode: 0b0100011,
        };
        assert_eq!(inst_ty, expect);
    }

    #[test]
    fn test_assembler_i_addi() {
        let s: &str = "addi 6, 16, 10\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let mut a = Assembler::new(&mut p);
        let inst_ty = a.assemble().unwrap().ty;
        let expect = I {
            imm: 10,
            rs1: 16,
            funct3: 0b000,
            rd: 6,
            opcode: 0b010011,
        };
        assert_eq!(inst_ty, expect);
    }

    #[test]
    fn test_assembler_r_add() {
        let s: &str = "add 0, 10, 5\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let mut a = Assembler::new(&mut p);
        let inst_ty = a.assemble().unwrap().ty;
        let expect = R {
            funct7: 0b0000000,
            rs2: 5,
            rs1: 10,
            funct3: 0b000,
            rd: 0,
            opcode: 0b0110011,
        };
        assert_eq!(inst_ty, expect);
    }

    #[test]
    fn test_assembler_r_sub() {
        let s: &str = "sub 1, 11, 6\n";
        let mut l = Lexer::new(s);
        let mut p = Parser::new(&mut l);
        let mut a = Assembler::new(&mut p);
        let inst = a.assemble().unwrap().ty;
        let expect = R {
            funct7: 0b0100000,
            rs2: 6,
            rs1: 11,
            funct3: 0,
            rd: 1,
            opcode: 0b0110011,
        };

        assert_eq!(inst, expect);
    }
}
