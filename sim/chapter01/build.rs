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

    let nand = nand::Nand::new("nand", &c);
    sim::generate(nand.m, sim::GenerationOptions::default(), &file);

    let and = and::And::new("and", &c);
    sim::generate(and.m, sim::GenerationOptions::default(), &file);

    let or = or::Or::new("or", &c);
    sim::generate(or.m, sim::GenerationOptions::default(), &file);

    let not = not::Not::new("not", &c);
    sim::generate(not.m, sim::GenerationOptions::default(), &file);

    let xor = xor::Xor::new("xor", &c);
    sim::generate(xor.m, sim::GenerationOptions::default(), &file);

    let mux = mux::Mux::new("mux", &c);
    sim::generate(mux.m, sim::GenerationOptions::default(), &file);

    let dmux = dmux::DMux::new("dmux", &c);
    sim::generate(dmux.m, sim::GenerationOptions::default(), &file);

    let and16 = and16::And16::new("and16", &c);
    sim::generate(and16.m, sim::GenerationOptions::default(), &file);

    let not16 = not16::Not16::new("not16", &c);
    sim::generate(not16.m, sim::GenerationOptions::default(), &file);

    let or16 = or16::Or16::new("or16", &c);
    sim::generate(or16.m, sim::GenerationOptions::default(), &file);

    let mux16 = mux16::Mux16::new("mux16", &c);
    sim::generate(mux16.m, sim::GenerationOptions::default(), &file);

    let or8way = or8way::Or8Way::new("or8way", &c);
    sim::generate(or8way.m, sim::GenerationOptions::default(), &file);

    let mux4way16 = mux4way16::Mux4Way16::new("mux4way16", &c);
    sim::generate(mux4way16.m, sim::GenerationOptions::default(), &file);

    let mux8way16 = mux8way16::Mux8Way16::new("mux8way16", &c);
    sim::generate(mux8way16.m, sim::GenerationOptions::default(), &file);

    let dmux4way = dmux4way::DMux4Way::new("dmux4way", &c);
    sim::generate(dmux4way.m, sim::GenerationOptions::default(), &file);

    let dmux8way = dmux8way::DMux8Way::new("dmux8way", &c);
    sim::generate(dmux8way.m, sim::GenerationOptions::default(), &file)
}
