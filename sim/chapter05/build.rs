use kaze::*;
use rtl::*;

use std::env;
use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("src");
    let dest_path = out_dir.join("modules.rs");
    let file = File::create(&dest_path).unwrap();

    let c = Context::new();

    let cpu = cpu::CPU::new("cpu", &c);
    sim::generate(cpu.m, sim::GenerationOptions::default(), &file);

    Ok(())
}
