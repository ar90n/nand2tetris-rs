use kaze::*;

use super::ram64::RAM64;
use super::dmux8way::DMux8Way;
use super::mux8way16::Mux8Way16;

pub struct RAM512<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub address: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> RAM512<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "RAM512");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);
        let address = m.input("address", 9);

        let dmux8way = DMux8Way::new("dmux8way", m);
        dmux8way.in_.drive(load);
        dmux8way.sel.drive(address.bits(address.bit_width() - 1, address.bit_width() - 3));

        let ram64_0 = RAM64::new("RAM64_0", m);
        ram64_0.in_.drive(in_);
        ram64_0.load.drive(dmux8way.a);
        ram64_0.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_1 = RAM64::new("RAM64_1", m);
        ram64_1.in_.drive(in_);
        ram64_1.load.drive(dmux8way.b);
        ram64_1.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_2 = RAM64::new("RAM64_2", m);
        ram64_2.in_.drive(in_);
        ram64_2.load.drive(dmux8way.c);
        ram64_2.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_3 = RAM64::new("RAM64_3", m);
        ram64_3.in_.drive(in_);
        ram64_3.load.drive(dmux8way.d);
        ram64_3.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_4 = RAM64::new("RAM64_4", m);
        ram64_4.in_.drive(in_);
        ram64_4.load.drive(dmux8way.e);
        ram64_4.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_5 = RAM64::new("RAM64_5", m);
        ram64_5.in_.drive(in_);
        ram64_5.load.drive(dmux8way.f);
        ram64_5.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_6 = RAM64::new("RAM64_6", m);
        ram64_6.in_.drive(in_);
        ram64_6.load.drive(dmux8way.g);
        ram64_6.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram64_7 = RAM64::new("RAM64_7", m);
        ram64_7.in_.drive(in_);
        ram64_7.load.drive(dmux8way.h);
        ram64_7.address.drive(address.bits(address.bit_width() - 4, 0));

        let mux8way16 = Mux8Way16::new("mux8way16", m);
        mux8way16.sel.drive(address.bits(address.bit_width() - 1, address.bit_width() - 3));
        mux8way16.a.drive(ram64_0.out);
        mux8way16.b.drive(ram64_1.out);
        mux8way16.c.drive(ram64_2.out);
        mux8way16.d.drive(ram64_3.out);
        mux8way16.e.drive(ram64_4.out);
        mux8way16.f.drive(ram64_5.out);
        mux8way16.g.drive(ram64_6.out);
        mux8way16.h.drive(ram64_7.out);

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