use kaze::*;

use super::ram512::RAM512;
use super::dmux8way::DMux8Way;
use super::mux8way16::Mux8Way16;

pub struct RAM4K<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub address: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> RAM4K<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "RAM4K");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);
        let address = m.input("address", 12);

        let dmux8way = DMux8Way::new("dmux8way", m);
        dmux8way.in_.drive(load);
        dmux8way.sel.drive(address.bits(address.bit_width() - 1, address.bit_width() - 3));

        let ram512_0 = RAM512::new("RAM512_0", m);
        ram512_0.in_.drive(in_);
        ram512_0.load.drive(dmux8way.a);
        ram512_0.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_1 = RAM512::new("RAM512_1", m);
        ram512_1.in_.drive(in_);
        ram512_1.load.drive(dmux8way.b);
        ram512_1.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_2 = RAM512::new("RAM512_2", m);
        ram512_2.in_.drive(in_);
        ram512_2.load.drive(dmux8way.c);
        ram512_2.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_3 = RAM512::new("RAM512_3", m);
        ram512_3.in_.drive(in_);
        ram512_3.load.drive(dmux8way.d);
        ram512_3.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_4 = RAM512::new("RAM512_4", m);
        ram512_4.in_.drive(in_);
        ram512_4.load.drive(dmux8way.e);
        ram512_4.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_5 = RAM512::new("RAM512_5", m);
        ram512_5.in_.drive(in_);
        ram512_5.load.drive(dmux8way.f);
        ram512_5.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_6 = RAM512::new("RAM512_6", m);
        ram512_6.in_.drive(in_);
        ram512_6.load.drive(dmux8way.g);
        ram512_6.address.drive(address.bits(address.bit_width() - 4, 0));

        let ram512_7 = RAM512::new("RAM512_7", m);
        ram512_7.in_.drive(in_);
        ram512_7.load.drive(dmux8way.h);
        ram512_7.address.drive(address.bits(address.bit_width() - 4, 0));

        let mux8way16 = Mux8Way16::new("mux8way16", m);
        mux8way16.sel.drive(address.bits(address.bit_width() - 1, address.bit_width() - 3));
        mux8way16.a.drive(ram512_0.out);
        mux8way16.b.drive(ram512_1.out);
        mux8way16.c.drive(ram512_2.out);
        mux8way16.d.drive(ram512_3.out);
        mux8way16.e.drive(ram512_4.out);
        mux8way16.f.drive(ram512_5.out);
        mux8way16.g.drive(ram512_6.out);
        mux8way16.h.drive(ram512_7.out);

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