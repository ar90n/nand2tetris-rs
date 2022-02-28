use std::fs::{self, File};
use std::io::{Read, Write};
use std::vec::Vec;
use std::{path::PathBuf, str::FromStr};

use anyhow::*;
use assembler::assembler;
use clap::Parser;
use compiler::compiler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    VM,
    Binary,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "vm" => Ok(Self::VM),
            "bin" => Ok(Self::Binary),
            _ => Err(anyhow::anyhow!("Unknown output format: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Args {
    #[clap(short, long)]
    input: PathBuf,

    #[clap(short, long)]
    output_format: OutputFormat,
}

fn find_input_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path]);
    }
    if path.is_dir() {
        return Ok(fs::read_dir(path)?
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file() && path.extension().unwrap() == "jack")
            .collect());
    }

    Err(anyhow!("{} is not a file or directory", path.display()))
}

struct FileContent {
    path: PathBuf,
    content: String,
}

impl FileContent {
    fn new(path: PathBuf, content: String) -> Self {
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

fn read_inputs(path: PathBuf) -> Result<Vec<FileContent>> {
    let input_file_paths = find_input_files(path)?;

    let mut ret = vec![];
    for input_file_path in input_file_paths {
        let mut file = File::open(&input_file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        ret.push(FileContent::new(input_file_path, content));
    }

    Ok(ret)
}

fn compile_to_vm(contents: &[FileContent]) -> Result<Vec<FileContent>> {
    let mut ret = vec![];
    for content in contents {
        let filename = content.filename()?;
        let commands = compiler::compile_to_vm(&content.content, filename)?;
        let output_content = commands
            .iter()
            .map(|c| c.dump())
            .collect::<Vec<_>>()
            .join("\n");
        let output_path = content.path.with_extension("vm");
        ret.push(FileContent::new(output_path, output_content));
    }

    Ok(ret)
}

fn compile_to_binary(contents: &[FileContent]) -> Result<String> {
    let mut hack_commands = vec![];
    for content in contents {
        let filename = content.filename()?;
        hack_commands.extend(compiler::compile_to_hack(&content.content, filename)?);
    }

    let commands = assembler::code::assemble(&hack_commands)?;
    let output_content = commands
        .iter()
        .map(|c| format!("{:016b}", c))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(output_content)
}

fn main() {
    let args = Args::parse();
    let input_file_contents = read_inputs(args.input).unwrap();
    match args.output_format {
        OutputFormat::VM => {
            let contents = compile_to_vm(&input_file_contents).expect("failed to compile to vm");
            for content in contents.iter() {
                let mut file = File::create(&content.path).unwrap();
                file.write_all(content.content.as_bytes()).unwrap();
            }
        }
        OutputFormat::Binary => {
            let content =
                compile_to_binary(&input_file_contents).expect("failed to compile to binary");
            let output_dir_path = if args.input.is_dir() {
                args.input.to_str().unwrap()
            } else {
                args.input
                    .parent()
                    .and_then(|path| path.to_str())
                    .unwrap_or("./")
            };
            let output_path = format!("{}/{}", output_dir_path, "Main.bin");
            let mut file = File::create(&output_path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    };
}
