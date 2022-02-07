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

        let (a, b, c, d) = if cfg!(feature = "builtin") {
            (
                m.output("a", if_(sel.eq(m.lit(0u32, 2u32)), in_).else_(m.low())),
                m.output("b", if_(sel.eq(m.lit(1u32, 2u32)), in_).else_(m.low())),
                m.output("c", if_(sel.eq(m.lit(2u32, 2u32)), in_).else_(m.low())),
                m.output("d", if_(sel.eq(m.lit(3u32, 2u32)), in_).else_(m.low())),
            )
        } else {
            let dmux1 = DMux::new("dmux1", m);
            dmux1.in_.drive(in_);
            dmux1.sel.drive(sel.bit(1));

            let dmux_ab = DMux::new("dmux_ab", m);
            dmux_ab.in_.drive(dmux1.a);
            dmux_ab.sel.drive(sel.bit(0));

            let dmux_cd = DMux::new("dmux_cd", m);
            dmux_cd.in_.drive(dmux1.b);
            dmux_cd.sel.drive(sel.bit(0));

            (
                m.output("a", dmux_ab.a),
                m.output("b", dmux_ab.b),
                m.output("c", dmux_cd.a),
                m.output("d", dmux_cd.b),
            )
        };

        Self {
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
