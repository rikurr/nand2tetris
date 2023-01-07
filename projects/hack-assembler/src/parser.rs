use crate::command::{ACommand, CCommand, Command};

pub struct Parser {
    asm_command: String,
}
impl Parser {
    pub fn new(asm_command: String) -> Self {
        Self { asm_command }
    }

    pub fn parse(&self) -> Vec<Command> {
        let parser = self
            .asm_command
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

                    return match value.parse::<u16>() {
                        Ok(v) => Command::A(ACommand { literal: v }),
                        Err(_) => Command::A(ACommand { literal: 10 }),
                    };
                }

                // ラベル処理
                if line.starts_with('(') {
                    return Command::L;
                }

                // C命令処理
                Command::C(CCommand::new(line.to_string()))
            })
            .collect::<Vec<Command>>();

        parser
    }
}
