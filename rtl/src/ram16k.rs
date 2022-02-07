use kaze::*;

use super::ram4k::RAM4K;
use super::dmux4way::DMux4Way;
use super::mux4way16::Mux4Way16;

pub struct RAM16K<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub address: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> RAM16K<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "RAM16K");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);
        let address = m.input("address", 14);

        let dmux4way = DMux4Way::new("dmux4way", m);
        dmux4way.in_.drive(load);
        dmux4way.sel.drive(address.bits(address.bit_width() - 1, address.bit_width() - 2));

        let ram4k_0 = RAM4K::new("RAM4K_0", m);
        ram4k_0.in_.drive(in_);
        ram4k_0.load.drive(dmux4way.a);
        ram4k_0.address.drive(address.bits(address.bit_width() - 3, 0));

        let ram4k_1 = RAM4K::new("RAM4K_1", m);
        ram4k_1.in_.drive(in_);
        ram4k_1.load.drive(dmux4way.b);
        ram4k_1.address.drive(address.bits(address.bit_width() - 3, 0));

        let ram4k_2 = RAM4K::new("RAM4K_2", m);
        ram4k_2.in_.drive(in_);
        ram4k_2.load.drive(dmux4way.c);
        ram4k_2.address.drive(address.bits(address.bit_width() - 3, 0));

        let ram4k_3 = RAM4K::new("RAM4K_3", m);
        ram4k_3.in_.drive(in_);
        ram4k_3.load.drive(dmux4way.d);
        ram4k_3.address.drive(address.bits(address.bit_width() - 3, 0));

        let mux4way16 = Mux4Way16::new("mux4way16", m);
        mux4way16.sel.drive(address.bits(address.bit_width() - 1, address.bit_width() - 2));
        mux4way16.a.drive(ram4k_0.out);
        mux4way16.b.drive(ram4k_1.out);
        mux4way16.c.drive(ram4k_2.out);
        mux4way16.d.drive(ram4k_3.out);

        let out = m.output("out", mux4way16.out);
        Self {
            m,
            in_,
            load,
            address,
            out,
        }
    }
}