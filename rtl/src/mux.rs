use kaze::*;

use super::and::*;
use super::not::*;
use super::or::*;

pub struct Mux<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Mux<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Mux");

        let a = m.input("a", 1);
        let b = m.input("b", 1);
        let sel = m.input("sel", 1);

        let not1 = Not::new("not1", m);
        not1.in_.drive(sel);
        let inv_sel = not1.out;

        let and1 = And::new("and1", m);
        and1.a.drive(a);
        and1.b.drive(inv_sel);
        let sel_a = and1.out;

        let and2 = And::new("and2", m);
        and2.a.drive(b);
        and2.b.drive(sel);
        let sel_b = and2.out;

        let or1 = Or::new("or1", m);
        or1.a.drive(sel_a);
        or1.b.drive(sel_b);
        let out = m.output("out", or1.out);

        Self { m, a, b, sel, out }
    }
}
