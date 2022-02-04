use super::not::*;
use kaze::*;

pub struct Not16<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Not16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Not16");

        let in_ = m.input("in_", 16);

        let acc = (0..16).map(|i| {
            let not = Not::new(format!("not{}", i), m);
            not.in_.drive(in_.bit(i));
            not.out.bit(0)
        }).reduce(|acc, out| {
            out.concat(acc)
        }).unwrap();
        let out = m.output("out", acc);

        Self { m, in_, out }
    }
}
