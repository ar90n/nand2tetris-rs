use kaze::*;

use super::add16::*;

pub struct Inc16<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Inc16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Inc16");

        let in_ = m.input("in_", 16);
        let l1 = m.lit(0x0001u16, 16);

        let add16 = Add16::new("add16", m);
        add16.a.drive(in_);
        add16.b.drive(l1);

        let out = m.output("out", add16.out);

        Self { m, in_, out }
    }
}