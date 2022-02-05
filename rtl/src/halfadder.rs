use kaze::*;

use super::and::*;
use super::xor::*;

pub struct HalfAdder<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub sum: &'a Output<'a>,
    pub carry: &'a Output<'a>,
}

impl<'a> HalfAdder<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "HalfAdder");

        let a = m.input("a", 1);
        let b = m.input("b", 1);

        let xor1 = Xor::new("xor1", m);
        xor1.a.drive(a);
        xor1.b.drive(b);

        let and1 =  And::new("and1", m);
        and1.a.drive(a);
        and1.b.drive(b);

        let sum = m.output("sum", xor1.out);
        let carry = m.output("carry", and1.out);

        Self { m, a, b, sum, carry }
    }
}