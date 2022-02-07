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

    let ram512 = ram512::RAM512::new("ram512", &c);
    sim::generate(ram512.m, sim::GenerationOptions::default(), &file);

    let ram4k = ram4k::RAM4K::new("ram4K", &c);
    sim::generate(ram4k.m, sim::GenerationOptions::default(), &file);

    //let ram16k = ram16k::RAM16K::new("ram16K", &c);
    //sim::generate(ram16k.m, sim::GenerationOptions::default(), &file);

    Ok(())
}