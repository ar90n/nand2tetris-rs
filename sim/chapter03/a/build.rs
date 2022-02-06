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

    let dff = dff::DFF::new("dff", &c);
    sim::generate(dff.m, sim::GenerationOptions::default(), &file);

    let bit = bit::Bit::new("bit", &c);
    sim::generate(bit.m, sim::GenerationOptions::default(), &file);

    let register = register::Register::new("register", &c);
    sim::generate(register.m, sim::GenerationOptions::default(), &file);

    let pc = pc::PC::new("pc", &c);
    sim::generate(pc.m, sim::GenerationOptions::default(), &file);

    let ram8 = ram8::RAM8::new("ram8", &c);
    sim::generate(ram8.m, sim::GenerationOptions::default(), &file);

    let ram64 = ram64::RAM64::new("ram64", &c);
    sim::generate(ram64.m, sim::GenerationOptions::default(), &file);

    Ok(())
}