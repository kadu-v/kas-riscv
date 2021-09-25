#[derive(Debug, PartialEq, Eq)]
pub enum AsmKind {
    EOASM,
    LW { imm: isize, rs1: isize, rd: isize },
    SW { imm: isize, rs2: isize, rs1: isize },
    ADDI { imm: isize, rs1: isize, rd: isize },
    ADD { rs2: isize, rs1: isize, rd: isize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    pub kind: AsmKind,
}
