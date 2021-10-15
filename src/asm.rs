use std::isize;

#[derive(Debug, PartialEq, Eq)]
pub enum AsmKind {
    EOASM,
    LW { imm: isize, rs1: isize, rd: isize },
    SW { imm: isize, rs2: isize, rs1: isize },
    ADDI { imm: isize, rs1: isize, rd: isize },
    ADD { rs2: isize, rs1: isize, rd: isize },
    SUB { rs2: isize, rs1: isize, rd: isize },
    AND { rs2: isize, rs1: isize, rd: isize },
    OR { rs2: isize, rs1: isize, rd: isize },
    XOR { rs2: isize, rs1: isize, rd: isize },
    SLT { rs2: isize, rs1: isize, rd: isize },
    SLTU { rs2: isize, rs1: isize, rd: isize },
    SLTI { imm: isize, rs1: isize, rd: isize },
    SLTIU { imm: isize, rs1: isize, rd: isize },
    SLL { rs2: isize, rs1: isize, rd: isize },
    SRL { rs2: isize, rs1: isize, rd: isize },
    SRA { rs2: isize, rs1: isize, rd: isize },
    SLLI { imm: isize, rs1: isize, rd: isize },
    SRLI { imm: isize, rs1: isize, rd: isize },
    SRAI { imm: isize, rs1: isize, rd: isize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    pub kind: AsmKind,
}
