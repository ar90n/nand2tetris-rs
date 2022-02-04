use anyhow;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::result::Result;
use std::vec::Vec;
use std::{any, fmt};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    cmp: PathBuf,

    #[clap(short, long)]
    tst: PathBuf,
}

#[derive(Debug, Clone, Copy)]
struct Signal {
    v: u64,
    w: u64,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.w == 1 {
            let v = if self.v == 1 { true } else { false };
            write!(f, "{}", v)
        } else {
            write!(f, "{}", self.v)
        }
    }
}

#[derive(Debug)]
struct CmpTable {
    inner: HashMap<String, Vec<Signal>>,
}

fn parse_signal(s: &str) -> anyhow::Result<Signal> {
    let mut s = s.trim();
    if 2 < s.len() && &s[0..2] == "%B" {
        s = &s[2..s.len()]
    }

    let w = s.len() as u64;
    let v = isize::from_str_radix(s, 2)? as u64;
    Ok(Signal { v, w })
}

fn parse_cmp(r: &mut impl Read) -> anyhow::Result<CmpTable> {
    let mut buf_reader = BufReader::new(r);
    let keys = {
        let mut buf = String::new();
        buf_reader.read_line(&mut buf)?;
        buf.split("|")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>()
    };
    let mut inner = keys
        .iter()
        .map(|k| (k.clone(), Vec::new()))
        .collect::<HashMap<_, _>>();

    buf_reader.lines().for_each(|l| {
        if let Result::Ok(buf) = l {
            for (i, s) in buf.split("|").filter(|s| !s.trim().is_empty()).enumerate() {
                let k = &keys[i];
                let signal = parse_signal(s).expect("invalid signal");
                inner.get_mut(k).unwrap().push(signal);
            }
        }
    });
    let table = CmpTable { inner };
    Ok(table)
}

#[derive(Debug)]
enum Ir {
    Load(String),
    Set(String),
    Eval,
    Output,
}

fn parse_tst(r: &mut impl Read) -> anyhow::Result<Vec<Ir>> {
    let instructions = BufReader::new(r)
        .lines()
        .into_iter()
        .filter_map(|s| {
            if let Result::Ok(s) = s {
                let tokens = s
                    .trim_end_matches(|c| c == ',' || c == ';')
                    .split(" ")
                    .collect::<Vec<_>>();

                match tokens[0] {
                    "load" => Some(Ir::Load(tokens[1].to_string())),
                    "set" => {
                        Some(Ir::Set(tokens[1].to_string()))
                    }
                    "eval" => Some(Ir::Eval),
                    "output" => Some(Ir::Output),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(instructions)
}

fn escape_label(s: &str) -> String {
    if s == "in" {
        return "in_".to_string();
    }

    return s.to_string();
}

struct CodeGenerator {
    cmp_table: CmpTable,
    output: Vec<String>,
    index: usize,
    module_name: String,
}

impl CodeGenerator {
    fn new(cmp_table: CmpTable) -> Self {
        Self {
            cmp_table,
            output: Vec::new(),
            index: 0,
            module_name: String::default(),
        }
    }

    fn execute(&mut self, ir: Ir) -> anyhow::Result<()> {
        match ir {
            Ir::Load(filename) => {
                self.module_name = filename
                    .chars()
                    .take_while(|&c| c != '.')
                    .collect::<String>();
            }
            Ir::Set(k) => {
                let label = escape_label(&k);
                let signal = self.cmp_table.inner.get(&k).unwrap()[self.index];
                self.output.push(format!("m.{} = {};", label, signal));
            }
            Ir::Eval => self.output.push("m.prop();".to_string()),
            Ir::Output => {
                for (k, v) in self.cmp_table.inner.iter() {
                    let label = escape_label(&k);
                    let expected = &v[self.index];
                    self.output
                        .push(format!("assert_eq!(m.{}, {});", label, expected));
                }
                self.index += 1;
            }
        }
        Ok(())
    }

    fn generate(&self) -> anyhow::Result<String> {
        let mut output = String::new();

        output.push_str("#[cfg(test)]\n");
        output.push_str("mod tests {\n");
        output.push_str("use super::super::modules::*;\n");
        output.push_str("#[test]\n");
        output.push_str(&format!("fn test_{}()", self.module_name));
        output.push_str("{\n");
        output.push_str(&format!("let mut m = {}::new();\n", self.module_name));
        output.push_str(&self.output.join("\n"));
        output.push_str("\n");
        output.push_str("}\n");
        output.push_str("}");
        Ok(output)
    }
}

fn main() {
    let args = Args::parse();
    let cmp_path = args.cmp.as_path();
    let mut file = File::open(cmp_path).expect(&format!("couldn't open {}", cmp_path.display()));
    let cmp_table = parse_cmp(&mut file).unwrap();

    let tst_path = args.tst.as_path();
    let mut file = File::open(tst_path).expect(&format!("couldn't open {}", tst_path.display()));
    let tst_seq = parse_tst(&mut file).unwrap();

    let mut generator = CodeGenerator::new(cmp_table);
    for ir in tst_seq {
        generator.execute(ir).unwrap();
    }
    let output = generator.generate().unwrap();
    println!("{}", output);
}
