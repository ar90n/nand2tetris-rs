use super::or::*;
use kaze::*;

pub struct Or16<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Or16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Or16");

        let a = m.input("a", 16);
        let b = m.input("b", 16);

        let acc = (0..16).map(|i| {
            let or = Or::new(format!("or{}", i), m);
            or.a.drive(a.bit(i));
            or.b.drive(b.bit(i));
            or.out.bit(0)
        }).reduce(|acc, out| {
            out.concat(acc)
        }).unwrap();
        let out = m.output("out", acc);

        Self { m, a, b, out }
    }
}
