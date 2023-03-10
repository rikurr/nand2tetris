use crate::command::{ACommand, Command, Comp, Dest, Jump};
use std::collections::HashMap;

// 機械語を出力する
pub fn code_generate(parser: Vec<Command>, symbol_table: HashMap<String, u16>) -> String {
    let code = parser
        .iter()
        .map(|parse| match parse {
            Command::A(a) => {
                let address = match a {
                    ACommand::Literal(literal) => literal,
                    ACommand::Mnemonic(mnemonic) => symbol_table
                        .get(*mnemonic)
                        .unwrap_or_else(|| panic!("Unresolved symbol: {}", mnemonic)),
                };

                format!("{:016b}", address)
            }

            Command::C(c) => {
                let dest = match c.dest {
                    Some(Dest::M) => "001",
                    Some(Dest::D) => "010",
                    Some(Dest::MD) => "011",
                    Some(Dest::A) => "100",
                    Some(Dest::AM) => "101",
                    Some(Dest::AD) => "110",
                    Some(Dest::AMD) => "111",
                    None => "000",
                };
                let comp = match c.comp {
                    Comp::Zero => "0101010",
                    Comp::One => "0111111",
                    Comp::MinusOne => "0111010",
                    Comp::DRegister => "0001100",
                    Comp::ARegister => "0110000",
                    Comp::NotDRegister => "0001101",
                    Comp::NotARegister => "0110001",
                    Comp::MinusDRegister => "0001111",
                    Comp::MinusARegister => "0110011",
                    Comp::DRegisterPlusOne => "0011111",
                    Comp::ARegisterPlusOne => "0110111",
                    Comp::DRegisterMinusOne => "0001110",
                    Comp::ARegisterMinusOne => "0110010",
                    Comp::DRegisterPlusARegister => "0000010",
                    Comp::DRegisterMinusARegister => "0010011",
                    Comp::ARegisterMinusDRegister => "0000111",
                    Comp::DRegisterAndARegister => "0000000",
                    Comp::DRegisterOrARegister => "0010101",
                    Comp::RAM => "1110000",
                    Comp::NotRAM => "1110001",
                    Comp::MinusRAM => "1110011",
                    Comp::RAMPlusOne => "1110111",
                    Comp::RAMMinusOne => "1110010",
                    Comp::DRegisterPlusRAM => "1000010",
                    Comp::DRegisterMinusRAM => "1010011",
                    Comp::RAMMinusDRegister => "1000111",
                    Comp::DRegisterAndRAM => "1000000",
                    Comp::DRegisterOrRAM => "1010101",
                };

                let jump = match c.jump {
                    Some(Jump::Jgt) => "001",
                    Some(Jump::Jeq) => "010",
                    Some(Jump::Jge) => "011",
                    Some(Jump::Jlt) => "100",
                    Some(Jump::Jne) => "101",
                    Some(Jump::Jle) => "110",
                    Some(Jump::Jmp) => "111",
                    None => "000",
                };
                format!("111{}{}{}", comp, dest, jump)
            }
            Command::L(_) => String::new(),
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join("\n");

    code
}
