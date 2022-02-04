use super::nand::*;
use kaze::*;

pub struct And<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> And<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "And");

        let a = m.input("a", 1);
        let b = m.input("b", 1);

        let nand1 = Nand::new("nand1", m);
        nand1.a.drive(a);
        nand1.b.drive(b);

        let nand2 = Nand::new("nand2", m);
        nand2.a.drive(nand1.out);
        nand2.b.drive(nand1.out);

        let out = m.output("out", nand2.out);

        Self { m, a, b, out }
    }
}
