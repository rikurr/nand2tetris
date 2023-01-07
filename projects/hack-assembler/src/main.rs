pub mod code;
pub mod command;
pub mod parser;

use code::Code;
use parser::Parser;

use std::{
    fs::{self},
    io::{self, Read},
};

fn main() -> std::io::Result<()> {
    let asm_file = fs::File::open("./test/add/Add.asm")?;
    let mut buf_reader = io::BufReader::new(asm_file);
    let mut asm_command = String::new();
    buf_reader.read_to_string(&mut asm_command)?;

    // Parser
    let parser = Parser::new(asm_command).parse();
    // Code Generate
    let machine_code = Code::new(parser).convert_binary();

    fs::write("./test/add/Add.Hack", b"opensource.com")?;
    println!("{}", machine_code);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_symbol() {
        let result = r#"
0000000000000010
1110110000010000
0000000000000011
1110000010010000
0000000000000000
1110001100001000
        "#
        .trim();

        let mut asm_command = r#"
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
        "#
        .to_string();

        println!("{}", result);

        // Parser
        let parser = Parser::new(asm_command).parse();

        // Code Generate
        let machine_code = Code::new(parser).convert_binary();
        assert_eq!(result, machine_code);
    }
}
