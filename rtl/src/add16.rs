use kaze::*;

use super::fulladder::*;

pub struct Add16<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Add16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Add16");

        let a = m.input("a", 16);
        let b = m.input("b", 16);

        let acc = (0..16).scan(m.lit(0u32, 1u32), |carry, i| {
            let fulladder = FullAdder::new(format!("fulladder{}", i), m);
            fulladder.a.drive(a.bit(i));
            fulladder.b.drive(b.bit(i));
            fulladder.c.drive(*carry);
            *carry = fulladder.carry.bit(0);
            Some(fulladder.sum.bit(0))
        }).reduce(|acc, out| {
            out.concat(acc)
        }).unwrap();

        let out = m.output("out", acc);

        Self {m, a, b, out}
    }
}