use super::dmux::*;
use kaze::*;

pub struct DMux4Way<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub a: &'a Output<'a>,
    pub b: &'a Output<'a>,
    pub c: &'a Output<'a>,
    pub d: &'a Output<'a>,
}

impl<'a> DMux4Way<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "DMux4Way");

        let in_ = m.input("in_", 1);
        let sel = m.input("sel", 2);

        let dmux1 = DMux::new("dmux1", m);
        dmux1.in_.drive(in_);
        dmux1.sel.drive(sel.bit(1));

        let dmux_ab = DMux::new("dmux_ab", m);
        dmux_ab.in_.drive(dmux1.a);
        dmux_ab.sel.drive(sel.bit(0));

        let dmux_cd = DMux::new("dmux_cd", m);
        dmux_cd.in_.drive(dmux1.b);
        dmux_cd.sel.drive(sel.bit(0));


        let a = m.output("a", dmux_ab.a);
        let b = m.output("b", dmux_ab.b);
        let c = m.output("c", dmux_cd.a);
        let d = m.output("d", dmux_cd.b);

        Self{
            m,
            in_,
            sel,
            a,
            b,
            c,
            d,
        }
    }
}
