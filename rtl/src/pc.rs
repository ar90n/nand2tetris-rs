use kaze::*;

use super::mux16::Mux16;
use super::inc16::Inc16;
use super::and16::And16;
use super::not::Not;
use super::or::Or;
use super::register::Register as Reg16;

pub struct PC<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub reset: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub inc: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> PC<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "PC");

        let in_ = m.input("in_", 16);
        let reset = m.input("reset", 1);
        let load = m.input("load", 1);
        let inc = m.input("inc", 1);

        let inc16 = Inc16::new("inc16", m);
        let mux16_in_inc = Mux16::new("mux16_in_inc", m);
        mux16_in_inc.a.drive(inc16.out);
        mux16_in_inc.b.drive(in_);
        mux16_in_inc.sel.drive(load);

        let inv_reset = Not::new("inv_reset", m);
        inv_reset.in_.drive(reset);

        let and16_reset = And16::new("and16_reset", m);
        and16_reset.a.drive(inv_reset.out.repeat(16));
        and16_reset.b.drive(mux16_in_inc.out);

        let or_reset_load = Or::new("or_reset_load", m);
        or_reset_load.a.drive(reset);
        or_reset_load.b.drive(load);

        let reg_load = Or::new("reg_load", m);
        reg_load.a.drive(or_reset_load.out);
        reg_load.b.drive(inc);

        let reg16 = Reg16::new("reg16", m);
        reg16.in_.drive(and16_reset.out);
        reg16.load.drive(reg_load.out);

        inc16.in_.drive(reg16.out);

        let out = m.output("out", reg16.out);

        Self { m, in_, reset, load, inc, out }
    }
}