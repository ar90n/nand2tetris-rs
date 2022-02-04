use super::dmux::*;
use super::dmux4way::*;
use kaze::*;

pub struct DMux8Way<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub a: &'a Output<'a>,
    pub b: &'a Output<'a>,
    pub c: &'a Output<'a>,
    pub d: &'a Output<'a>,
    pub e: &'a Output<'a>,
    pub f: &'a Output<'a>,
    pub g: &'a Output<'a>,
    pub h: &'a Output<'a>,
}

impl<'a> DMux8Way<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "DMux8Way");

        let in_ = m.input("in_", 1);
        let sel = m.input("sel", 3);

        let dmux1 = DMux::new("dmux1", m);
        dmux1.in_.drive(in_);
        dmux1.sel.drive(sel.bit(2));

        let dmux_abcd = DMux4Way::new("dmux_abcd", m);
        dmux_abcd.in_.drive(dmux1.a);
        dmux_abcd.sel.drive(sel.bits(1,0));

        let dmux_efgh = DMux4Way::new("dmux_efgh", m);
        dmux_efgh.in_.drive(dmux1.b);
        dmux_efgh.sel.drive(sel.bits(1,0));


        let a = m.output("a", dmux_abcd.a);
        let b = m.output("b", dmux_abcd.b);
        let c = m.output("c", dmux_abcd.c);
        let d = m.output("d", dmux_abcd.d);
        let e = m.output("e", dmux_efgh.a);
        let f = m.output("f", dmux_efgh.b);
        let g = m.output("g", dmux_efgh.c);
        let h = m.output("h", dmux_efgh.d);

        Self{
            m,
            in_,
            sel,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        }
    }
}
