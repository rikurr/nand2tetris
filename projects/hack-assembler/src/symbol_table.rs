use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    pub next_ram_address: u16,
    pub symbols: HashMap<String, u16>,
    pub pc: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut symbols: HashMap<String, u16> = HashMap::new();

        // ユーザー定義変数の初期アドレス位置
        let next_ram_address: u16 = 15;

        symbols.insert("R0".to_string(), 0);
        symbols.insert("R1".to_string(), 1);
        symbols.insert("R2".to_string(), 2);
        symbols.insert("R3".to_string(), 3);
        symbols.insert("R4".to_string(), 4);
        symbols.insert("R5".to_string(), 5);
        symbols.insert("R6".to_string(), 6);
        symbols.insert("R7".to_string(), 7);
        symbols.insert("R8".to_string(), 8);
        symbols.insert("R9".to_string(), 9);
        symbols.insert("R10".to_string(), 10);
        symbols.insert("R11".to_string(), 11);
        symbols.insert("R12".to_string(), 12);
        symbols.insert("R13".to_string(), 13);
        symbols.insert("R14".to_string(), 14);
        symbols.insert("R15".to_string(), 15);
        symbols.insert("SCREEN".to_string(), 16384);
        symbols.insert("KBD".to_string(), 24576);
        symbols.insert("SP".to_string(), 0);
        symbols.insert("LCL".to_string(), 1);
        symbols.insert("ARG".to_string(), 2);
        symbols.insert("THIS".to_string(), 3);
        symbols.insert("THAT".to_string(), 4);

        Self {
            next_ram_address,
            symbols,
            pc: 0,
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        println!("シンボル追加{}", &symbol);
        self.symbols.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> u16 {
        *self.symbols.get(symbol).unwrap()
    }
}
