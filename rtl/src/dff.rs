use kaze::*;

pub struct DFF<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> DFF<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "DFF");

        let in_ = m.input("in_", 1);
        let reg = m.reg("reg", 1);

        reg.drive_next(in_);

        let out = m.output("out", reg);

        Self { m, in_, out }
    }
}