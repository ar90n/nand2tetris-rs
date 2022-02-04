use kaze::*;

pub struct Nand<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Nand<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Nand");

        let a = m.input("a", 1);
        let b = m.input("b", 1);
        let out = m.output("out", !(a & b));

        Self { m, a, b, out }
    }
}
