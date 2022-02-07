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

        let (a, b, c, d, e, f, g, h) = if cfg!(feature = "builtin") {
            (
                m.output("a", if_(sel.eq(m.lit(0u32, 3u32)), in_).else_(m.low())),
                m.output("b", if_(sel.eq(m.lit(1u32, 3u32)), in_).else_(m.low())),
                m.output("c", if_(sel.eq(m.lit(2u32, 3u32)), in_).else_(m.low())),
                m.output("d", if_(sel.eq(m.lit(3u32, 3u32)), in_).else_(m.low())),
                m.output("e", if_(sel.eq(m.lit(4u32, 3u32)), in_).else_(m.low())),
                m.output("f", if_(sel.eq(m.lit(5u32, 3u32)), in_).else_(m.low())),
                m.output("g", if_(sel.eq(m.lit(6u32, 3u32)), in_).else_(m.low())),
                m.output("h", if_(sel.eq(m.lit(7u32, 3u32)), in_).else_(m.low())),
            )
        } else {
            let dmux1 = DMux::new("dmux1", m);
            dmux1.in_.drive(in_);
            dmux1.sel.drive(sel.bit(2));

            let dmux_abcd = DMux4Way::new("dmux_abcd", m);
            dmux_abcd.in_.drive(dmux1.a);
            dmux_abcd.sel.drive(sel.bits(1, 0));

            let dmux_efgh = DMux4Way::new("dmux_efgh", m);
            dmux_efgh.in_.drive(dmux1.b);
            dmux_efgh.sel.drive(sel.bits(1, 0));

            (
                m.output("a", dmux_abcd.a),
                m.output("b", dmux_abcd.b),
                m.output("c", dmux_abcd.c),
                m.output("d", dmux_abcd.d),
                m.output("e", dmux_efgh.a),
                m.output("f", dmux_efgh.b),
                m.output("g", dmux_efgh.c),
                m.output("h", dmux_efgh.d),
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
            e,
            f,
            g,
            h,
        }
    }
}
