use kaze::*;

use super::halfadder::*;
use super::or::*;


pub struct FullAdder<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub c: &'a Input<'a>,
    pub sum: &'a Output<'a>,
    pub carry: &'a Output<'a>,
}

impl<'a> FullAdder<'a> {
    pub fn new(internal_signal: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(internal_signal, "FullAdder");

        let a = m.input("a", 1);
        let b = m.input("b", 1);
        let c = m.input("c", 1);

        let half_adder1 = HalfAdder::new("half_adder1", m);
        half_adder1.a.drive(a);
        half_adder1.b.drive(b);

        let half_adder2 = HalfAdder::new("half_adder2", m);
        half_adder2.a.drive(half_adder1.sum);
        half_adder2.b.drive(c);

        let or1 = Or::new("or1", m);
        or1.a.drive(half_adder1.carry);
        or1.b.drive(half_adder2.carry);

        let sum = m.output("sum", half_adder2.sum);
        let carry = m.output("carry", or1.out);

        Self { m, a, b, c, sum, carry }
    }
}