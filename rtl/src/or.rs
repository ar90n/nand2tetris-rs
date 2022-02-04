use super::nand::*;
use kaze::*;

pub struct Or<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Or<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Or");

        let a = m.input("a", 1);
        let b = m.input("b", 1);

        let nand1 = Nand::new("nand1", m);
        nand1.a.drive(a);
        nand1.b.drive(a);

        let nand2 = Nand::new("nand2", m);
        nand2.a.drive(b);
        nand2.b.drive(b);

        let nand3 = Nand::new("nand3", m);
        nand3.a.drive(nand1.out);
        nand3.b.drive(nand2.out);

        let out = m.output("out", nand3.out);

        Self { m, a, b, out }
    }
}
