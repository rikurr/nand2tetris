use crate::command::{ACommand, CCommand, Command, Comp, Dest, Jump};
use crate::symbol_table::SymbolTable;

pub fn second_parse(
    asm_command: &str,
    mut symbol_table: SymbolTable,
) -> (Vec<Command>, SymbolTable) {
    let parser = asm_command
        .split('\n')
        .filter(|line| {
            let line = line.trim().split('/').next().unwrap().trim();
            !line.is_empty()
        })
        .map(|line| {
            // 前後の空白の削除とコメントを取り除く
            let line = line.trim().split('/').next().unwrap().trim();

            // A命令の処理
            if line.starts_with('@') {
                let value = line.strip_prefix('@').unwrap();

                // 数値にパースできるかどうか
                return match value.parse::<u16>() {
                    Ok(num) => Command::A(ACommand::Literal(num)),
                    Err(_) => {
                        if !symbol_table.contains(value) {
                            symbol_table.next_ram_address += 1;
                            symbol_table.add_entry(value.to_string(), symbol_table.next_ram_address)
                        };
                        Command::A(ACommand::Mnemonic(value))
                    }
                };
            }

            // ラベル処理
            if line.starts_with('(') {
                let value = line.trim_start_matches('(').trim_end_matches(')');
                return Command::L(value.to_string());
            }

            // C命令処理
            let dest = if line.contains('=') {
                let dest_mnemonic = line.split('=').next().unwrap();

                match dest_mnemonic {
                    "M" => Some(Dest::M),
                    "D" => Some(Dest::D),
                    "MD" => Some(Dest::MD),
                    "A" => Some(Dest::A),
                    "AM" => Some(Dest::AM),
                    "AD" => Some(Dest::AD),
                    "AMD" => Some(Dest::AMD),
                    _ => panic!("Unknown computation instruction: {}", dest_mnemonic),
                }
            } else {
                None
            };

            let jump = if line.contains(';') {
                let jump_mnemonic = line.split(';').nth(1).unwrap();

                match jump_mnemonic {
                    "JGT" => Some(Jump::Jgt),
                    "JEQ" => Some(Jump::Jeq),
                    "JGE" => Some(Jump::Jge),
                    "JLT" => Some(Jump::Jlt),
                    "JNE" => Some(Jump::Jne),
                    "JLE" => Some(Jump::Jle),
                    "JMP" => Some(Jump::Jmp),
                    _ => panic!("Unknown computation instruction: {}", jump_mnemonic),
                }
            } else {
                None
            };

            let comp_mnemonic = if line.contains('=') {
                line.split('=').nth(1).unwrap()
            } else if line.contains(';') {
                line.split(';').next().unwrap()
            } else {
                line
            };

            let comp = match comp_mnemonic {
                "0" => Comp::Zero,
                "1" => Comp::One,
                "-1" => Comp::MinusOne,
                "D" => Comp::DRegister,
                "A" => Comp::ARegister,
                "M" => Comp::RAM,
                "!D" => Comp::NotDRegister,
                "!A" => Comp::NotARegister,
                "!M" => Comp::NotRAM,
                "-D" => Comp::MinusDRegister,
                "-A" => Comp::MinusARegister,
                "-M" => Comp::MinusRAM,
                "D+1" => Comp::DRegisterPlusOne,
                "A+1" => Comp::ARegisterPlusOne,
                "M+1" => Comp::RAMPlusOne,
                "D-1" => Comp::DRegisterMinusOne,
                "A-1" => Comp::ARegisterMinusOne,
                "M-1" => Comp::RAMMinusOne,
                "D+A" => Comp::DRegisterPlusARegister,
                "D+M" => Comp::DRegisterPlusRAM,
                "D-A" => Comp::DRegisterMinusARegister,
                "D-M" => Comp::DRegisterMinusRAM,
                "A-D" => Comp::ARegisterMinusDRegister,
                "M-D" => Comp::RAMMinusDRegister,
                "D&A" => Comp::DRegisterAndARegister,
                "D&M" => Comp::DRegisterAndRAM,
                "D|A" => Comp::DRegisterOrARegister,
                "D|M" => Comp::DRegisterOrRAM,
                _ => panic!("Unknown computation instruction: {}", comp_mnemonic),
            };

            Command::C(CCommand { dest, comp, jump })
        })
        .collect::<Vec<Command>>();

    (parser, symbol_table)
}

pub fn first_parse(asm_command: &str, mut symbol_table: SymbolTable) -> SymbolTable {
    asm_command
        .split('\n')
        .filter(|line| {
            let line = line.trim().split('/').next().unwrap().trim();
            !line.is_empty()
        })
        .for_each(|line| {
            let line = line.trim().split('/').next().unwrap().trim();
            println!("line: {}", line);

            // A命令の処理
            if line.starts_with('@') {
                symbol_table.pc += 1;
                return;
            }

            // ラベル処理
            if line.starts_with('(') {
                let mnemonic = line
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .to_string();
                symbol_table.add_entry(mnemonic, symbol_table.pc);
                return;
            }

            symbol_table.pc += 1
        });

    symbol_table
}
