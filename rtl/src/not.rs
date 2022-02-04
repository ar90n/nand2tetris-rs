use super::nand::*;
use kaze::*;

pub struct Not<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Not<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Not");

        let in_ = m.input("in_", 1);

        let nand1 = Nand::new("nand1", m);
        nand1.a.drive(in_);
        nand1.b.drive(in_);

        let out = m.output("out", nand1.out);

        Self { m, in_, out }
    }
}
