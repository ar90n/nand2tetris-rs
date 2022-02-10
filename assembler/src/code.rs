use std::collections::HashMap;

use super::command::{Command, Comp, Dest, Jump};

struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new(program: &[Command]) -> Self {
        let mut table = vec![
            ("SP".to_string(), 0),
            ("LCL".to_string(), 1),
            ("ARG".to_string(), 2),
            ("THIS".to_string(), 3),
            ("THAT".to_string(), 4),
            ("R0".to_string(), 0),
            ("R1".to_string(), 1),
            ("R2".to_string(), 2),
            ("R3".to_string(), 3),
            ("R4".to_string(), 4),
            ("R5".to_string(), 5),
            ("R6".to_string(), 6),
            ("R7".to_string(), 7),
            ("R8".to_string(), 8),
            ("R9".to_string(), 9),
            ("R10".to_string(), 10),
            ("R11".to_string(), 11),
            ("R12".to_string(), 12),
            ("R13".to_string(), 13),
            ("R14".to_string(), 14),
            ("R15".to_string(), 15),
            ("SCREEN".to_string(), 16384),
            ("KBD".to_string(), 24576),
        ].into_iter().collect::<HashMap::<_, _>>();
        let mut pc = 0;
        for command in program.iter() {
            match command {
                Command::L(label) => {
                    table.insert(label.clone(), pc);
                },
                _ => pc += 1,
            }
        }
        let mut address = 0x10;
        for command in program.iter() {
            if let Command::ASymbol(label) = command {
                if !table.contains_key(label) {
                    table.insert(label.clone(), address);
                    address += 1;
                }
            }
        }
        Self { table }
    }

    pub fn get(&self, label: &str) -> Option<u16> {
        self.table.get(label).copied()
    }
}

fn assemble_command(command: &Command, symbol_table: &SymbolTable) -> anyhow::Result<Option<u16>> {
    match command {
        Command::AImm(imm) => Ok(Some(*imm)),
        Command::ASymbol(symbol) => {
            if let Some(v) = symbol_table.get(symbol) {
                Ok(Some(v))
            } else {
                Err(anyhow::anyhow!("Undefined symbol: {}", symbol))
            }
        }
        Command::C(dest, comp, jump) => {
            let bits = 0xe000 | (comp.assemble() << 6) | (dest.assemble() << 3) | jump.assemble();
            Ok(Some(bits))
        }
        _ => Ok(None),
    }
}

pub fn parse(lines: &[String]) -> anyhow::Result<Vec<Command>> {
    lines.iter()
        .filter(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with("//")
        })
        .map(|s| Command::parse(&s))
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn assemble(program: &[Command]) -> anyhow::Result<Vec<u16>> {
    let symbol_table = SymbolTable::new(program);

    let mut result = Vec::new();
    for command in program {
        if let Some(bits) = assemble_command(command, &symbol_table)? {
            result.push(bits);
        }
    }
    Ok(result)
}