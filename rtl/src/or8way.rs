use super::or::*;
use kaze::*;

pub struct Or8Way<'a> {
    pub m: &'a Module<'a>,
    pub in_: &'a Input<'a>,
    pub out: &'a Output<'a>,
}

impl<'a> Or8Way<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "Or8Way");

        let in_ = m.input("in_", 8);

        let or01 = Or::new("or01", m);
        or01.a.drive(in_.bit(0));
        or01.b.drive(in_.bit(1));

        let or23 = Or::new("or23", m);
        or23.a.drive(in_.bit(2));
        or23.b.drive(in_.bit(3));

        let or45 = Or::new("or45", m);
        or45.a.drive(in_.bit(4));
        or45.b.drive(in_.bit(5));

        let or67 = Or::new("or67", m);
        or67.a.drive(in_.bit(6));
        or67.b.drive(in_.bit(7));

        let or0123 = Or::new("or0123", m);
        or0123.a.drive(or01.out);
        or0123.b.drive(or23.out);

        let or4567 = Or::new("or4567", m);
        or4567.a.drive(or45.out);
        or4567.b.drive(or67.out);

        let or01234567 = Or::new("or01234567", m);
        or01234567.a.drive(or0123.out);
        or01234567.b.drive(or4567.out);

        let out = m.output("out", or01234567.out);

        Self { m, in_, out }
    }
}
