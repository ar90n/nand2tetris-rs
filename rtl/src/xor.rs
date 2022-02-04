use super::nand::*;
use super::not::*;
use kaze::*;

pub struct Xor<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Xor<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Xor");

        let a = m.input("a", 1);
        let b = m.input("b", 1);

        let not1 = Not::new("not1", m);
        not1.in_.drive(a);
        let inv_a = not1.out;

        let not2 = Not::new("not2", m);
        not2.in_.drive(b);
        let inv_b = not2.out;

        let nand1 = Nand::new("nand1", m);
        nand1.a.drive(a);
        nand1.b.drive(inv_b);

        let nand2 = Nand::new("nand2", m);
        nand2.a.drive(b);
        nand2.b.drive(inv_a);

        let nand3 = Nand::new("nand3", m);
        nand3.a.drive(nand1.out);
        nand3.b.drive(nand2.out);

        let out = m.output("out", nand3.out);

        Self { m, a, b, out }
    }
}
