use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum InstType {
    EOINST,
    // I形式の命令
    // | 31 ----------------- 20 | 19 --- 15 | 14 --- 12 | 11 --- 7 | 6 --- 0 |
    //       imm[11:0]                rs1        funct3        rd      opcode
    I {
        imm: usize,
        rs1: usize,
        funct3: usize,
        rd: usize,
        opcode: usize,
    },
    // S形式の命令
    // | 31 ----- 25 | 24 --- 20 | 19 --- 15 | 14 --- 12 | 11 --- 7 | 6 --- 0 |
    //    imm[11:5]       rs2         rs1        funct3    imm[4:0]   opcode
    S {
        imm_1: usize,
        rs2: usize,
        rs1: usize,
        funct3: usize,
        imm_2: usize,
        opcode: usize,
    },

    // R形式の命令
    // | 31 ----- 25 | 24 --- 20 | 19 --- 15 | 14 --- 12 | 11 --- 7 | 6 --- 0 |
    //    funct7          rs2          rs1      funct3        rd       opcode
    R {
        funct7: usize,
        rs2: usize,
        rs1: usize,
        funct3: usize,
        rd: usize,
        opcode: usize,
    },
}

impl Display for InstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::EOINST => {
                write!(f, "EINST")
            }
            Self::I {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            } => {
                write!(f, "I {{ imm: {:#014b}, rs1: {:#07b}, funct3: {:#05b}, rd: {:#07b}, opcode: {:#09b} }}", imm, rs1, funct3, rd, opcode)
            }
            Self::S {
                imm_1,
                rs2,
                rs1,
                funct3,
                imm_2,
                opcode,
            } => {
                write!(f, "S {{ imm_1: {:#09b}, rs2: {:#07b}, rs1: {:#07b}, funct3: {:#05b}, imm_2: {:#07b}, opcode: {:#09b} }}", imm_1, rs2, rs1, funct3, imm_2, opcode)
            }
            Self::R {
                funct7,
                rs2,
                rs1,
                funct3,
                rd,
                opcode,
            } => {
                write!(f, "R {{ funct7: {:#09b}, rs2: {:#07b}, rs1: {:#07b}, funct3: {:#05b}, rd: {:#07b}, opcode: {:#09b} }}", funct7, rs2, rs1, funct3, rd, opcode)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Inst {
    pub ty: InstType,
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = format!("{}", self.ty);
        write!(f, "Inst {{ ty: {} }}", s)
    }
}
