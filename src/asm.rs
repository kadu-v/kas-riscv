use std::isize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmKind {
    EOASM,
    LABEL {
        l: String,
    },
    LW {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    SW {
        imm: isize,
        rs2: isize,
        rs1: isize,
    },
    ADDI {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    ADD {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SUB {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    AND {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    OR {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    XOR {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SLT {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SLTU {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SLTI {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    SLTIU {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    SLL {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SRL {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SRA {
        rs2: isize,
        rs1: isize,
        rd: isize,
    },
    SLLI {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    SRLI {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    SRAI {
        imm: isize,
        rs1: isize,
        rd: isize,
    },
    BEQ {
        imm: Option<isize>,
        rs1: isize,
        rd: isize,
        label: Option<String>,
    },
    BNE {
        imm: Option<isize>,
        rs1: isize,
        rd: isize,
        label: Option<String>,
    },
    BLT {
        imm: Option<isize>,
        rs1: isize,
        rd: isize,
        label: Option<String>,
    },
    BGE {
        imm: Option<isize>,
        rs1: isize,
        rd: isize,
        label: Option<String>,
    },
    BLTU {
        imm: Option<isize>,
        rs1: isize,
        rd: isize,
        label: Option<String>,
    },
    BGEU {
        imm: Option<isize>,
        rs1: isize,
        rd: isize,
        label: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asm {
    pub kind: AsmKind,
}
