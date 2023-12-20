use std::collections::HashMap;

pub struct SymbolTable {
    symbols: HashMap<String, u16>,
    next_id: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn get(&mut self, symbol: &str) -> u16 {
        if let Some(id) = self.symbols.get(symbol) {
            *id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.symbols.insert(symbol.to_string(), id);
            id
        }
    }

    pub fn len(&self) -> usize {
        self.next_id as usize
    }
}
