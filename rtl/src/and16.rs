use super::and::*;
use kaze::*;

pub struct And16<'a> {
    pub m: &'a Module<'a>,
    pub a: &'a Input<'a>,
    pub b: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> And16<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "And16");

        let a = m.input("a", 16);
        let b = m.input("b", 16);

        let out = if cfg!(feature = "builtin") {
            m.output("out", a & b)
        } else {
            let acc = (0..16)
                .map(|i| {
                    let and = And::new(format!("and{}", i), m);
                    and.a.drive(a.bit(i));
                    and.b.drive(b.bit(i));
                    and.out.bit(0)
                })
                .reduce(|acc, out| out.concat(acc))
                .unwrap();
            m.output("out", acc)
        };

        Self { m, a, b, out }
    }
}
