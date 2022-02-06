use kaze::*;

use super::ram8::RAM8;
use super::dmux8way::DMux8Way;
use super::mux8way16::Mux8Way16;

pub struct RAM64<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub address: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> RAM64<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "RAM64");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);
        let address = m.input("address", 6);

        let dmux8way = DMux8Way::new("dmux8way", m);
        dmux8way.in_.drive(load);
        dmux8way.sel.drive(address.bits(5, 3));

        let ram8_0 = RAM8::new("RAM8_0", m);
        ram8_0.in_.drive(in_);
        ram8_0.load.drive(dmux8way.a);
        ram8_0.address.drive(address.bits(2, 0));

        let ram8_1 = RAM8::new("RAM8_1", m);
        ram8_1.in_.drive(in_);
        ram8_1.load.drive(dmux8way.b);
        ram8_1.address.drive(address.bits(2, 0));

        let ram8_2 = RAM8::new("RAM8_2", m);
        ram8_2.in_.drive(in_);
        ram8_2.load.drive(dmux8way.c);
        ram8_2.address.drive(address.bits(2, 0));

        let ram8_3 = RAM8::new("RAM8_3", m);
        ram8_3.in_.drive(in_);
        ram8_3.load.drive(dmux8way.d);
        ram8_3.address.drive(address.bits(2, 0));

        let ram8_4 = RAM8::new("RAM8_4", m);
        ram8_4.in_.drive(in_);
        ram8_4.load.drive(dmux8way.e);
        ram8_4.address.drive(address.bits(2, 0));

        let ram8_5 = RAM8::new("RAM8_5", m);
        ram8_5.in_.drive(in_);
        ram8_5.load.drive(dmux8way.f);
        ram8_5.address.drive(address.bits(2, 0));

        let ram8_6 = RAM8::new("RAM8_6", m);
        ram8_6.in_.drive(in_);
        ram8_6.load.drive(dmux8way.g);
        ram8_6.address.drive(address.bits(2, 0));

        let ram8_7 = RAM8::new("RAM8_7", m);
        ram8_7.in_.drive(in_);
        ram8_7.load.drive(dmux8way.h);
        ram8_7.address.drive(address.bits(2, 0));

        let mux8way16 = Mux8Way16::new("mux8way16", m);
        mux8way16.sel.drive(address.bits(5,3));
        mux8way16.a.drive(ram8_0.out);
        mux8way16.b.drive(ram8_1.out);
        mux8way16.c.drive(ram8_2.out);
        mux8way16.d.drive(ram8_3.out);
        mux8way16.e.drive(ram8_4.out);
        mux8way16.f.drive(ram8_5.out);
        mux8way16.g.drive(ram8_6.out);
        mux8way16.h.drive(ram8_7.out);

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