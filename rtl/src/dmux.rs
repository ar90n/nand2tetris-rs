use kaze::*;

use super::not::*;
use super::and::*;

pub struct DMux<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub a: &'a Output<'a>,
    pub b: &'a Output<'a>,
}

impl<'a> DMux<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "DMux");

        let in_ = m.input("in_", 1);
        let sel = m.input("sel", 1);

        let not1 = Not::new("not1", m);
        not1.in_.drive(sel);
        let inv_sel = not1.out;

        let and1 = And::new("and1", m);
        and1.a.drive(in_);
        and1.b.drive(inv_sel);
        let a = m.output("a", and1.out);

        let and2 = And::new("and2", m);
        and2.a.drive(in_);
        and2.b.drive(sel);
        let b = m.output("b", and2.out);

        Self { m, in_, sel, a, b}
    }
}
