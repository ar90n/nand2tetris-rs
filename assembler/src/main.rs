use anyhow;
use std::{io, io::prelude::*};

use assembler::code::{assemble, parse};
use assembler::command::{Command, Comp, Dest, Jump};

pub fn main() -> anyhow::Result<()> {
    let lines = io::stdin()
        .lock()
        .lines()
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let commands = parse(&lines)?;
    let binary = assemble(&commands)?;

    for word in binary {
        println!("{:016b}", word);
    }
    Ok(())
}
