use kaze::*;

use super::bit::*;

pub struct Register<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Register<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Register");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);

        let acc = (0..16).map(|i| {
            let bit = Bit::new(format!("bit{}", i), m);
            bit.in_.drive(in_.bit(i));
            bit.load.drive(load);
            bit.out.bit(0)
        }).reduce(|acc, out| {
            out.concat(acc)
        }).unwrap();
        let out = m.output("out", acc);

        Self { m, in_, load, out }
    }
}