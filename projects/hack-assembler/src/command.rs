// A命令
// ex: @xxx
pub enum ACommand<'a> {
    Literal(u16),
    Mnemonic(&'a str),
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

// 命令
pub enum Command<'a> {
    A(ACommand<'a>),
    C(CCommand),
    L(String),
}
