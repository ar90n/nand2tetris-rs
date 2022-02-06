use kaze::*;

pub struct Register<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub load: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Register<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Register");

        let in_ = m.input("in_", 16);
        let load = m.input("load", 1);

        let out = if cfg!(feature = "builtin") {
            use super::mux16::Mux16;
            let reg = m.reg("reg", 16);
            let mux = Mux16::new("mux", m);
            mux.a.drive(reg);
            mux.b.drive(in_);
            mux.sel.drive(load);
            reg.drive_next(mux.out);

            m.output("out", reg)
        } else {
            use super::bit::Bit;
            let acc = (0..16)
                .map(|i| {
                    let bit = Bit::new(format!("bit{}", i), m);
                    bit.in_.drive(in_.bit(i));
                    bit.load.drive(load);
                    bit.out.bit(0)
                })
                .reduce(|acc, out| out.concat(acc))
                .unwrap();
            m.output("out", acc)
        };

        Self { m, in_, load, out }
    }
}
