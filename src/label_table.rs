use std::collections::HashMap;

use crate::asm::{Asm, AsmKind::*};
use crate::parser::Parser;

pub fn make_label_table<'a>(
    p: &'a mut Parser<'a>,
) -> Result<(Vec<Asm>, HashMap<String, isize>), String> {
    let mut vs = vec![];

    loop {
        let a = p.parse()?;
        if let EOASM = a.kind {
            vs.push(a);
            break;
        }
        vs.push(a);
    }

    let mut pc = 0;
    let mut label_table = HashMap::new();

    for a in vs.iter() {
        if let LABEL { l } = a.kind.clone() {
            label_table.insert(l, pc);
        }
        pc += 4;
    }

    Ok((vs, label_table))
}
