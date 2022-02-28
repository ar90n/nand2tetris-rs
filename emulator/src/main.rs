use std::fs;
use std::io::Read;
use std::ops::Index;
use std::path::PathBuf;

use anyhow::*;
use clap::Parser;

use assembler::code::assemble;
use assembler::command::Command as HackCommand;
use compiler::compiler::compile_to_hack;
use vm;

mod modules;

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Args {
    #[clap(short, long)]
    input: PathBuf,
}

pub struct Computer {
    pub reset: bool,

    cpu: modules::CPU,
    memory: Vec<u16>,
    rom: Vec<u16>,
}

impl Computer {
    pub fn new(program: Vec<u16>) -> Self {
        let cpu = modules::CPU::new();
        let memory = vec![0; 32768];
        let mut rom = vec![0; 65536];

        for (i, &word) in program.iter().enumerate() {
            rom[i] = word;
        }

        Self {
            reset: false,
            cpu,
            memory,
            rom,
        }
    }

    pub fn step(&mut self) {
        // update input
        self.cpu.reset = self.reset;
        self.cpu.prop();

        // update output
        self.cpu.instruction = self.rom[self.cpu.pc as usize] as u32;
        self.cpu.inM = self.memory[self.cpu.addressM as usize] as u32;
        self.cpu.prop();

        // update internal state
        self.cpu.posedge_clk();
        self.cpu.prop();
        dbg!(&self.cpu.pc);
    }
}

fn find_input_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path]);
    }
    if path.is_dir() {
        return Ok(fs::read_dir(path)?
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file() && (path.extension().unwrap() == "jack" || path.extension().unwrap() == "vm"))   
            .collect());
    }

    Err(anyhow!("{} is not a file or directory", path.display()))
}

#[derive(Debug, Clone)]
struct FileContent<T> {
    path: PathBuf,
    content: T,
}

impl<T> FileContent<T> {
    fn new(path: PathBuf, content: T) -> Self {
        Self { path, content }
    }

    fn filename(&self) -> Result<String> {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .context(format!("wrong file path: {:?}", &self.path))
    }
}

fn read_inputs(path: PathBuf) -> Result<Vec<FileContent<String>>> {
    let input_file_paths = find_input_files(path)?;

    let mut ret = vec![];
    for input_file_path in input_file_paths {
        let mut file = fs::File::open(&input_file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        ret.push(FileContent::new(input_file_path, content));
    }

    Ok(ret)
}

fn compile_vm(content: &FileContent<String>) -> Result<FileContent<Vec<HackCommand>>> {
    let filename = content.filename()?;
    let vm_commands = vm::code::parse(&content.content.split("\n").collect::<Vec<_>>())?;
    let hack_commands = vm::code::translate(&vm_commands, Some(&filename))?;
    Ok(FileContent::<_>::new(content.path.clone(), hack_commands))
}

fn compile_jack(content: &FileContent<String>) -> Result<FileContent<Vec<HackCommand>>> {
    let filename = content.filename()?;
    let hack_commands = compile_to_hack(&content.content, filename)?;
    Ok(FileContent::<_>::new(content.path.clone(), hack_commands))
}

fn compile(contents: &[FileContent<String>]) -> Result<Vec<u16>> {
    let mut hack_programs = contents
        .iter()
        .map(|content| match content.path.extension() {
            Some(ext) if ext == "vm" => compile_vm(content),
            Some(ext) if ext == "jack" => compile_jack(content),
            _ => Err(anyhow!("unknown file extension: {:?}", content.path)),
        })
        .collect::<Result<Vec<FileContent<Vec<HackCommand>>>>>()
        .expect("failed to compile Jack files");

    if let Some(index) = hack_programs
        .iter()
        .position(|content| match content.filename() {
            anyhow::Result::Ok(filename) => filename == "Sys.jack" || filename == "Sys.vm",
            Err(_) => false,
        })
    {
        hack_programs.swap(0, index);
    }

    let hack_commands = hack_programs
        .into_iter()
        .map(|c| c.content)
        .flatten()
        .collect::<Vec<HackCommand>>();
    assemble(&hack_commands)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_file_contents = read_inputs(args.input).expect("failed to read input files");
    let binary = compile(&input_file_contents)?;
    let mut computer = Computer::new(binary);

    loop {
        computer.step();
    }

    Ok(())
}
