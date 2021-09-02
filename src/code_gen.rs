
use crate::instructions::*;
use crate::instructions::InstType::*;


pub fn gen_hex(inst: &Inst) -> String {
    match inst.ty {
       I { imm, rs1, funct3, rd, opcode } => {
            let s = format!("{:012b}{:05b}{:03b}{:05b}{:07b}", imm, rs1, funct3, rd, opcode);
            let c7 = u8::from_str_radix(&s[0..4], 2).unwrap();
            let c6 = u8::from_str_radix(&s[4..8], 2).unwrap();
            let c5 = u8::from_str_radix(&s[8..12], 2).unwrap();
            let c4 = u8::from_str_radix(&s[12..16], 2).unwrap();
            let c3 = u8::from_str_radix(&s[16..20], 2).unwrap();
            let c2 = u8::from_str_radix(&s[20..24], 2).unwrap();
            let c1 = u8::from_str_radix(&s[24..28], 2).unwrap();
            let c0 = u8::from_str_radix(&s[28..32], 2).unwrap();
            format!("{:01x}{:01x}\n{:01x}{:01x}\n{:01x}{:01x}\n{:01x}{:01x}", c1, c0, c3, c2, c5, c4, c7, c6)
        }
    }
}