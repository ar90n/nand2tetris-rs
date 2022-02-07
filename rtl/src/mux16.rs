use super::mux::*;
use kaze::*;

pub struct Mux16<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub sel: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Mux16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Mux16");

        let a = m.input("a", 16);
        let b = m.input("b", 16);
        let sel = m.input("sel", 1);

        let out = if cfg!(feature = "builtin") {
            m.output("out", if_(sel.bit(0), b).else_(a))
        } else {
        let acc = (0..16).map(|i| {
            let mux = Mux::new(format!("mux{}", i), m);
            mux.a.drive(a.bit(i));
            mux.b.drive(b.bit(i));
            mux.sel.drive(sel);
            mux.out.bit(0)
        }).reduce(|acc, out| {
            out.concat(acc)
        }).unwrap();
        m.output("out", acc)
        };

        Self { m, a, b, sel, out }
    }
}
