use super::mux16::*;
use kaze::*;

pub struct Mux4Way16<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub c: &'a Input<'a>,
    pub d: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Mux4Way16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Mux4Way16");

        let a = m.input("a", 16);
        let b = m.input("b", 16);
        let c = m.input("c", 16);
        let d = m.input("d", 16);
        let sel = m.input("sel", 2);

        let mux16_ab = Mux16::new("mux16_ab", m);
        mux16_ab.a.drive(a);
        mux16_ab.b.drive(b);
        mux16_ab.sel.drive(sel.bit(0));

        let mux16_cd = Mux16::new("mux16_cd", m);
        mux16_cd.a.drive(c);
        mux16_cd.b.drive(d);
        mux16_cd.sel.drive(sel.bit(0));

        let mux16_abcd = Mux16::new("mux16_abcd", m);
        mux16_abcd.a.drive(mux16_ab.out);
        mux16_abcd.b.drive(mux16_cd.out);
        mux16_abcd.sel.drive(sel.bit(1));

        let out = m.output("out", mux16_abcd.out);
        Self{
            m,
            a,
            b,
            c,
            d,
            sel,
            out,
        }
    }
}
