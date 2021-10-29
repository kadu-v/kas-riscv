use kas_riscv::assembler::Assembler;
// use kas_riscv::assembler::{assemble_bin, assemble_hex};
use kas_riscv::code_gen::gen_hex;
use kas_riscv::lexer::Lexer;
use kas_riscv::parser::Parser;

use std::env;
use std::ffi::OsStr;
use std::fs::*;
use std::path::Path;
use std::process;

use std::io::{BufReader, Read, Write};

use kas_riscv::lexer::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("please input file!");
        process::exit(1);
    }

    let input_file_path = Path::new(&args[1]);
    let output_file_path = match input_file_path.extension().and_then(OsStr::to_str) {
        Some("kas") => input_file_path.with_extension("hex"),
        _ => {
            eprintln!("file extension is not *.kas !");
            process::exit(1);
        }
    };

    let input = match read_to_string(input_file_path) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(1);
        }
    };
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let mut a = Assembler::new(&mut p);
    let output = match a.assemble_all() {
        Ok(src) => src,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    match File::create(output_file_path) {
        Ok(mut output_file) => {
            if let Err(e) = write!(output_file, "{}", output) {
                eprintln!("{}", e.to_string());
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}
