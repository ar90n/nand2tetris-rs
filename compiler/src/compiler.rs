use super::dump_vm::Context;
use super::dump_vm::*;
use super::parser::parse;
use super::tokenizer::tokenize;
use assembler::code::assemble;
use assembler::command::Command as HackCommand;
use vm::code::translate;
use vm::command::Command as VmCommand;

use anyhow::*;

pub fn compile_to_vm(program: &str, filename: String) -> Result<Vec<VmCommand>> {
    let tokens = tokenize(program).unwrap();
    let class = parse(&tokens).unwrap();

    let mut context = Context::new(filename);
    Ok(class.dump_as_vm(&mut context))
}

pub fn compile_to_hack(program: &str, filename: String) -> Result<Vec<HackCommand>> {
    let vm_commands = compile_to_vm(program, filename.clone())?;
    translate(&vm_commands, Some(&filename))
}