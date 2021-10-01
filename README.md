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
                             v  ----------------------| aseembelr.rs |
                             |                        +--------------+
                   +---------+----------+  
                   | Instruction struct |  
                   +---------+----------+  
                             |                        +-------------+
                             v  ----------------------| code_gen.rs |
                             |                        +-------------+
                  +----------+----------+
                  | Binary code (riscv) |
                  +---------------------+
```
## How to install
```
$ git clone git@github.com:kadu-v/kas-riscv.git
$ 
```




## How to use
