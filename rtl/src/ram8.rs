use kaze::*;

use super::register::Register as Reg16;
use super::dmux8way::DMux8Way;
use super::mux8way16::Mux8Way16;

pub struct RAM8<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub address: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> RAM8<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "RAM8");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);
        let address = m.input("address", 3);

        let dmux8way = DMux8Way::new("dmux8way", m);
        dmux8way.in_.drive(load);
        dmux8way.sel.drive(address);

        let reg0 = Reg16::new("reg0", m);
        reg0.in_.drive(in_);
        reg0.load.drive(dmux8way.a);

        let reg1 = Reg16::new("reg1", m);
        reg1.in_.drive(in_);
        reg1.load.drive(dmux8way.b);

        let reg2 = Reg16::new("reg2", m);
        reg2.in_.drive(in_);
        reg2.load.drive(dmux8way.c);

        let reg3 = Reg16::new("reg3", m);
        reg3.in_.drive(in_);
        reg3.load.drive(dmux8way.d);

        let reg4 = Reg16::new("reg4", m);
        reg4.in_.drive(in_);
        reg4.load.drive(dmux8way.e);

        let reg5 = Reg16::new("reg5", m);
        reg5.in_.drive(in_);
        reg5.load.drive(dmux8way.f);

        let reg6 = Reg16::new("reg6", m);
        reg6.in_.drive(in_);
        reg6.load.drive(dmux8way.g);

        let reg7 = Reg16::new("reg7", m);
        reg7.in_.drive(in_);
        reg7.load.drive(dmux8way.h);

        let mux8way16 = Mux8Way16::new("mux8way16", m);
        mux8way16.sel.drive(address);
        mux8way16.a.drive(reg0.out);
        mux8way16.b.drive(reg1.out);
        mux8way16.c.drive(reg2.out);
        mux8way16.d.drive(reg3.out);
        mux8way16.e.drive(reg4.out);
        mux8way16.f.drive(reg5.out);
        mux8way16.g.drive(reg6.out);
        mux8way16.h.drive(reg7.out);

        let out = m.output("out", mux8way16.out);
        Self {
            m,
            in_,
            load,
            address,
            out,
        }
    }
}