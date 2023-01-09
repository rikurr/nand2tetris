pub mod code;
pub mod command;
pub mod parser;
pub mod symbol_table;

use std::{
    fs,
    io::{self, Read},
};

use crate::symbol_table::SymbolTable;

fn main() -> std::io::Result<()> {
    let asm_file = fs::File::open("./test/rect/Rect.asm")?;
    let mut buf_reader = io::BufReader::new(asm_file);
    let mut asm_command = String::new();
    buf_reader.read_to_string(&mut asm_command)?;

    // Parser Symbol

    // Parser

    let symbol_table = SymbolTable::new();
    let first_parse = parser::first_parse(&asm_command, symbol_table);
    let parse = parser::parse(&asm_command, first_parse);

    // Code Generate
    let machine_code = code::code_generate(parse.0, parse.1.symbols);

    fs::write("./test/rect/Rect.hack", &machine_code)?;
    println!("{}", &machine_code);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_not_symbol() {
        let result = r#"
0000000000000010
1110110000010000
0000000000000011
1110000010010000
0000000000000000
1110001100001000
        "#
        .trim();

        let asm_command = r#"
// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

@2
D=A
@3
D=D+A
@0
M=D
        "#;

        let symbol_table = SymbolTable::new();
        let first_parse = parser::first_parse(&asm_command, symbol_table);
        let parse = parser::parse(&asm_command, first_parse);

        // Code Generate
        let machine_code = code::code_generate(parse.0, parse.1.symbols);
        assert_eq!(result, machine_code);
    }

    #[test]
    fn test_parse() {
        let result = r#"
0000000000000000
1111110000010000
0000000000000001
1111010011010000
0000000000001010
1110001100000001
0000000000000001
1111110000010000
0000000000001100
1110101010000111
0000000000000000
1111110000010000
0000000000000010
1110001100001000
0000000000001110
1110101010000111
        "#
        .trim();

        let asm_command = r#"
        // This file is part of www.nand2tetris.org
        // and the book "The Elements of Computing Systems"
        // by Nisan and Schocken, MIT Press.
        // File name: projects/06/max/Max.asm
        
        // Computes R2 = max(R0, R1)  (R0,R1,R2 refer to RAM[0],RAM[1],RAM[2])
        
            @R0
            D=M              // D = first number
            @R1
            D=D-M            // D = first number - second number
            @OUTPUT_FIRST
            D;JGT            // if D>0 (first is greater) goto output_first
            @R1
            D=M              // D = second number
            @OUTPUT_D
            0;JMP            // goto output_d
            (OUTPUT_FIRST)
            @R0             
            D=M              // D = first number
            (OUTPUT_D)
            @R2
            M=D              // M[2] = D (greatest number)
            (INFINITE_LOOP)
            @INFINITE_LOOP
            0;JMP            // infinite loop
        "#;

        let symbol_table = SymbolTable::new();
        let first_parse = parser::first_parse(&asm_command, symbol_table);
        let parse = parser::parse(&asm_command, first_parse);

        // Code Generate
        let machine_code = code::code_generate(parse.0, parse.1.symbols);
        assert_eq!(result, machine_code);
    }
}
