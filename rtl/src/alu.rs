use kaze::*;

use super::add16::*;
use super::and16::*;
use super::mux16::*;
use super::not::*;
use super::not16::*;
use super::or::*;
use super::or8way::*;

pub struct ALU<'a> {
    pub m: &'a Module<'a>,
    pub x: &'a Input<'a>,
    pub y: &'a Input<'a>,
    pub zx: &'a Input<'a>,
    pub nx: &'a Input<'a>,
    pub zy: &'a Input<'a>,
    pub ny: &'a Input<'a>,
    pub f: &'a Input<'a>,
    pub no: &'a Input<'a>,
    pub out: &'a Output<'a>,
    pub zr: &'a Output<'a>,
    pub ng: &'a Output<'a>,
}

impl<'a> ALU<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        fn convert_input<'a>(
            instance_prefix: impl Into<String>,
            m: &'a Module<'a>,
            in_: &'a Input<'a>,
            z: &'a Input<'a>,
            n: &'a Input<'a>,
        ) -> &'a Output<'a> {
            let prefix = instance_prefix.into();
            let not = Not::new(format!("{}_not", prefix), m);
            not.in_.drive(z);

            let and16 = And16::new(format!("{}_and16", prefix), m);
            and16.a.drive(in_);
            and16.b.drive(not.out.repeat(16));

            let not16 = Not16::new(format!("{}_not16", prefix), m);
            not16.in_.drive(and16.out);

            let mux16 = Mux16::new(format!("{}_mux16", prefix), m);
            mux16.a.drive(and16.out);
            mux16.b.drive(not16.out);
            mux16.sel.drive(n);

            mux16.out
        }

        let m = p.module(instance_name, "ALU");

        let xin = m.input("x", 16);
        let zx = m.input("zx", 1);
        let nx = m.input("nx", 1);
        let x = convert_input("x", m, xin, zx, nx);

        let yin = m.input("y", 16);
        let zy = m.input("zy", 1);
        let ny = m.input("ny", 1);
        let y = convert_input("y", m, yin, zy, ny);

        let add16 = Add16::new("add16", m);
        add16.a.drive(x);
        add16.b.drive(y);

        let and16 = And16::new("and16", m);
        and16.a.drive(x);
        and16.b.drive(y);

        let f = m.input("f", 1);
        let mux16_1 = Mux16::new("mux16_1", m);
        mux16_1.a.drive(and16.out);
        mux16_1.b.drive(add16.out);
        mux16_1.sel.drive(f);

        let not16 = Not16::new("not16", m);
        not16.in_.drive(mux16_1.out);

        let no = m.input("no", 1);
        let mux16_2 = Mux16::new("mux16_2", m);
        mux16_2.a.drive(mux16_1.out);
        mux16_2.b.drive(not16.out);
        mux16_2.sel.drive(no);

        let out = m.output("out", mux16_2.out);

        let or8way_0_7 = Or8Way::new("or8way_0_7", m);
        or8way_0_7.in_.drive(mux16_2.out.bits(7, 0));

        let or8way_8_15 = Or8Way::new("or8way_8_15", m);
        or8way_8_15.in_.drive(mux16_2.out.bits(15, 8));

        let or = Or::new("or", m);
        or.a.drive(or8way_0_7.out);
        or.b.drive(or8way_8_15.out);
        let not = Not::new("not", m);
        not.in_.drive(or.out);
        let zr = m.output("zr", not.out);

        let ng = m.output("ng", mux16_2.out.bit(15));

        Self {
            m,
            x: xin,
            y: yin,
            zx,
            nx,
            zy,
            ny,
            f,
            no,
            out,
            zr,
            ng,
        }
    }
}
