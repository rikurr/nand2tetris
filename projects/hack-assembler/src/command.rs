// A命令
// ex: @xxx
pub struct ACommand {
    pub literal: u16,
}

pub enum Dest {
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

pub enum Comp {
    Zero,
    One,
    MinusOne,
    DRegister,
    ARegister,
    RAM,
    NotDRegister,
    NotARegister,
    NotRAM,
    MinusDRegister,
    MinusARegister,
    MinusRAM,
    DRegisterPlusOne,
    ARegisterPlusOne,
    RAMPlusOne,
    DRegisterMinusOne,
    ARegisterMinusOne,
    RAMMinusOne,
    DRegisterPlusARegister,
    DRegisterPlusRAM,
    DRegisterMinusARegister,
    DRegisterMinusRAM,
    ARegisterMinusDRegister,
    RAMMinusDRegister,
    DRegisterAndARegister,
    DRegisterAndRAM,
    DRegisterOrARegister,
    DRegisterOrRAM,
}

pub enum Jump {
    Jgt,
    Jeq,
    Jge,
    Jlt,
    Jne,
    Jle,
    Jmp,
}

/// C命令
/// ex: dest=comp;jump
pub struct CCommand {
    pub dest: Option<Dest>,
    pub comp: Comp,
    pub jump: Option<Jump>,
}

impl CCommand {
    pub fn new(line: String) -> Self {
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

        let jump = if line.contains('(') {
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
            &line
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

        Self { dest, comp, jump }
    }
}
// 命令
pub enum Command {
    A(ACommand),
    C(CCommand),
    L,
}
