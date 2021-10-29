use crate::inst::InstType::*;
use crate::inst::*;

// 命令形式に従って、バイナリを生成する
pub fn gen_bin(inst: &Inst) -> String {
    match inst.ty {
        EOINST => "".to_string(),
        I {
            imm,
            rs1,
            funct3,
            rd,
            opcode,
        } => format!(
            "{:012b}{:05b}{:03b}{:05b}{:07b}",
            imm, rs1, funct3, rd, opcode
        ),
        S {
            imm_1,
            rs2,
            rs1,
            funct3,
            imm_2,
            opcode,
        } => format!(
            "{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}",
            imm_1, rs2, rs1, funct3, imm_2, opcode
        ),
        R {
            funct7,
            rs2,
            rs1,
            funct3,
            rd,
            opcode,
        } => format!(
            "{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}",
            funct7, rs2, rs1, funct3, rd, opcode,
        ),
    }
}

// バイナリをhexに変換する
pub fn gen_hex(inst: &Inst) -> String {
    let s = gen_bin(&inst);
    let mut cs = [0, 0, 0, 0, 0, 0, 0, 0];
    cs[7] = u8::from_str_radix(&s[0..4], 2).unwrap();
    cs[6] = u8::from_str_radix(&s[4..8], 2).unwrap();
    cs[5] = u8::from_str_radix(&s[8..12], 2).unwrap();
    cs[4] = u8::from_str_radix(&s[12..16], 2).unwrap();
    cs[3] = u8::from_str_radix(&s[16..20], 2).unwrap();
    cs[2] = u8::from_str_radix(&s[20..24], 2).unwrap();
    cs[1] = u8::from_str_radix(&s[24..28], 2).unwrap();
    cs[0] = u8::from_str_radix(&s[28..32], 2).unwrap();
    for (i, c) in cs.iter().enumerate() {
        println!("{}: {}", i, c);
    }
    format!(
        "{:01x}{:01x}\n{:01x}{:01x}\n{:01x}{:01x}\n{:01x}{:01x}\n",
        cs[1], cs[0], cs[3], cs[2], cs[5], cs[4], cs[7], cs[6]
    )
}
