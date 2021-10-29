# kas-riscv

This is a minimal assembler for risc-v.  
これは、`はじめてのCPU自作` のための最小構成のアセンブラです。

## Overview
```
            +-------------------------------+  
            | source code (riscv assembler) |  
            +----------------+--------------+  
                             |                        +----------+   
                             v  ----------------------| lexer.rs | 
                             |                        +----------+   
                        +----+----+  
                        | Tokens  |  
                        +----+----+  
                             |                        +-----------+
                             v  ----------------------| parser.rs |
                             |                        +-----------+
                      +------+------+  
                      | Asm struct  |  
                      +------+------+  
                             |                        +--------------+
                             v  ----------------------| aseembler.rs |
                             |                        +--------------+
                      +------+------+  
                      | Inst struct |  
                      +------+------+  
                             |                        +-------------+
                             v  ----------------------| code_gen.rs |
                             |                        +-------------+
                  +----------+----------+
                  | Binary code (riscv) |
                  +---------------------+
```
## How to install/use
```
$ git clone git@github.com:kadu-v/kas-riscv.git
$ cd kas-riscv
$ cargo run ./sources/add.kas
```

`add.hex` file is generated in `source` directry
