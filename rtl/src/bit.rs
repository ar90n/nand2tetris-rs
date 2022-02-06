use kaze::*;

use super::dff::*;
use super::mux::*;

pub struct Bit<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Bit<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Bit");

        let in_ = m.input("in_", 1);
        let load = m.input("load", 1);

        let dff = DFF::new("dff", m);
        let mux = Mux::new("mux", m);
        mux.a.drive(dff.out);
        mux.b.drive(in_);
        mux.sel.drive(load);
        dff.in_.drive(mux.out);

        let out = m.output("out", dff.out);

        Self { m, in_, load, out }
    }
}