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

    let halfadder = halfadder::HalfAdder::new("halfadder", &c);
    sim::generate(halfadder.m, sim::GenerationOptions::default(), &file);

    let fulladder = fulladder::FullAdder::new("fulladder", &c);
    sim::generate(fulladder.m, sim::GenerationOptions::default(), &file);

    let add16 = add16::Add16::new("add16", &c);
    sim::generate(add16.m, sim::GenerationOptions::default(), &file);

    let inc16 = inc16::Inc16::new("inc16", &c);
    sim::generate(inc16.m, sim::GenerationOptions::default(), &file);

    let alu = alu::ALU::new("alu", &c);
    sim::generate(alu.m, sim::GenerationOptions::default(), &file);

    Ok(())
}
