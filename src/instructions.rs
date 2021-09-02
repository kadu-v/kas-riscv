use std::fmt::{Display, Formatter, Result};


#[derive(Debug, PartialEq, Eq)]
pub enum InstType {
    EOINST,
    // I形式の命令 
    // 31 ----- 20 | 19 - 15 | 14 - 12 | 11 - 7 | 6 - 0 |
    //  imm[11:0]      rs1      funct3     rd     opcode
    I { imm: u16, rs1: u8, funct3: u8, rd: u8, opcode: u8 },
}

impl Display for InstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            InstType::EOINST => {
                write!(f, "EINST")
            },
            InstType::I { imm, rs1, funct3, rd, opcode} => {
                write!(f, "I {{ imm: {:#014b}, rs1: {:#07b}, funct3: {:#05b}, rd: {:#07b}, opcode: {:#09b} }}", imm, rs1, funct3, rd, opcode)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Inst {
   pub ty : InstType
}


impl Display for Inst {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = format!("{}", self.ty);
        write!(f, "Inst {{ ty: {} }}", s)
    }
}