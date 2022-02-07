use super::mux16::*;
use super::mux4way16::*;
use kaze::*;

pub struct Mux8Way16<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub c: &'a Input<'a>,
    pub d: &'a Input<'a>,
    pub e: &'a Input<'a>,
    pub f: &'a Input<'a>,
    pub g: &'a Input<'a>,
    pub h: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Mux8Way16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Mux8Way16");

        let a = m.input("a", 16);
        let b = m.input("b", 16);
        let c = m.input("c", 16);
        let d = m.input("d", 16);
        let e = m.input("e", 16);
        let f = m.input("f", 16);
        let g = m.input("g", 16);
        let h = m.input("h", 16);
        let sel = m.input("sel", 3);

        let out = if cfg!(feature = "builtin") {
            let mux_out = if_(sel.eq(m.lit(0u32, 3u32)), a)
                .else_if(sel.eq(m.lit(1u32, 3u32)), b)
                .else_if(sel.eq(m.lit(2u32, 3u32)), c)
                .else_if(sel.eq(m.lit(3u32, 3u32)), d)
                .else_if(sel.eq(m.lit(4u32, 3u32)), e)
                .else_if(sel.eq(m.lit(5u32, 3u32)), f)
                .else_if(sel.eq(m.lit(6u32, 3u32)), g)
                .else_(h);
            m.output("out", mux_out)
        } else {
            let mux4way16_abcd = Mux4Way16::new("mux4way16_abcd", m);
            mux4way16_abcd.a.drive(a);
            mux4way16_abcd.b.drive(b);
            mux4way16_abcd.c.drive(c);
            mux4way16_abcd.d.drive(d);
            mux4way16_abcd.sel.drive(sel.bits(1, 0));

            let mux4way16_efgh = Mux4Way16::new("mux4way16_efgh", m);
            mux4way16_efgh.a.drive(e);
            mux4way16_efgh.b.drive(f);
            mux4way16_efgh.c.drive(g);
            mux4way16_efgh.d.drive(h);
            mux4way16_efgh.sel.drive(sel.bits(1, 0));

            let mux16 = Mux16::new("mux16", m);
            mux16.a.drive(mux4way16_abcd.out);
            mux16.b.drive(mux4way16_efgh.out);
            mux16.sel.drive(sel.bit(2));

            m.output("out", mux16.out)
        };

        Self {
            m,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            sel,
            out,
        }
    }
}
