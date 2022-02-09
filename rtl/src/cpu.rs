use kaze::*;

use super::alu::ALU;
use super::and::And;
use super::mux16::Mux16;
use super::not::Not;
use super::or::Or;
use super::pc::PC;
use super::register::Register;

pub struct CPU<'a> {
    pub m: &'a Module<'a>,
    pub inM: &'a Input<'a>,
    pub instruction: &'a Input<'a>,
    pub reset: &'a Input<'a>,
    pub outM: &'a Output<'a>,
    pub writeM: &'a Output<'a>,
    pub addressM: &'a Output<'a>,
    pub pc: &'a Output<'a>,
    pub DRegister: &'a Output<'a>,
    pub ARegister: &'a Output<'a>,
}

impl<'a> CPU<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Self {
        let m = p.module(instance_name, "CPU");

        let inM = m.input("inM", 16);
        let instruction = m.input("instruction", 16);
        let reset = m.input("reset", 1);

        let alu = ALU::new("alu", m);

        let (i, a, c, d, j) = (
            instruction.bit(15),
            instruction.bit(12),
            instruction.bits(11, 6),
            instruction.bits(5, 3),
            instruction.bits(2, 0),
        );

        let pc_load = {
            let and_j2_zr = And::new("and_j2_zr", m);
            and_j2_zr.a.drive(j.bit(1));
            and_j2_zr.b.drive(alu.zr);

            let and_j1_ng = And::new("and_j1_ng", m);
            and_j1_ng.a.drive(j.bit(2));
            and_j1_ng.b.drive(alu.ng);

            let or_pc_load1 = Or::new("or_pc_load1", m);
            or_pc_load1.a.drive(and_j2_zr.out);
            or_pc_load1.b.drive(and_j1_ng.out);

            let or_zr_ng = Or::new("or_zr_ng", m);
            or_zr_ng.a.drive(alu.zr);
            or_zr_ng.b.drive(alu.ng);

            let not_zr_or_ng = Not::new("not_zr_or_ng", m);
            not_zr_or_ng.in_.drive(or_zr_ng.out);

            let and_j3_not_zr_or_ng = And::new("and_j3_not_zr_or_ng", m);
            and_j3_not_zr_or_ng.a.drive(j.bit(0));
            and_j3_not_zr_or_ng.b.drive(not_zr_or_ng.out);

            let or_pc_load = Or::new("or_pc_load", m);
            or_pc_load.a.drive(or_pc_load1.out);
            or_pc_load.b.drive(and_j3_not_zr_or_ng.out);

            let gated_pc_load2 = And::new("gated_pc_load2", m);
            gated_pc_load2.a.drive(i);
            gated_pc_load2.b.drive(or_pc_load.out);

            gated_pc_load2.out
        };

        let mux_im_out = Mux16::new("mux_im_out", m);
        mux_im_out.a.drive(instruction);
        mux_im_out.b.drive(alu.out);
        mux_im_out.sel.drive(i);

        let a_reg = Register::new("a_reg", m);
        let a_reg_load = {
            let inv_i = Not::new("inv_i", m);
            inv_i.in_.drive(i);
            let or_i_reg_load = Or::new("or_i_reg_load", m);
            or_i_reg_load.a.drive(inv_i.out);
            or_i_reg_load.b.drive(d.bit(2));

            or_i_reg_load.out
        };
        a_reg.in_.drive(mux_im_out.out);
        a_reg.load.drive(a_reg_load);

        let pc = PC::new("pc", m);
        pc.in_.drive(a_reg.out);
        pc.load.drive(pc_load);
        pc.reset.drive(reset);
        pc.inc.drive(m.high());

        let mux_a_m = Mux16::new("mux_a_m", m);
        mux_a_m.a.drive(a_reg.out);
        mux_a_m.b.drive(inM);
        mux_a_m.sel.drive(a);

        let d_reg = Register::new("d_reg", m);
        let d_reg_load = {
            let and_i_d2 = And::new("and_i_d2", m);
            and_i_d2.a.drive(i);
            and_i_d2.b.drive(d.bit(1));
            and_i_d2.out
        };
        d_reg.in_.drive(alu.out);
        d_reg.load.drive(d_reg_load);

        alu.x.drive(d_reg.out);
        alu.y.drive(mux_a_m.out);
        alu.zx.drive(c.bit(5));
        alu.nx.drive(c.bit(4));
        alu.zy.drive(c.bit(3));
        alu.ny.drive(c.bit(2));
        alu.f.drive(c.bit(1));
        alu.no.drive(c.bit(0));

        let writeM = m.output("writeM", {
            let and_i_writeM = And::new("and_i_writeM", m);
            and_i_writeM.a.drive(i);
            and_i_writeM.b.drive(d.bit(0));
            and_i_writeM.out
        });
        let pc = m.output("pc", pc.out);
        let addressM = m.output("addressM", m.low().concat(a_reg.out.bits(14, 0)));
        let outM = m.output("outM", alu.out);
        let DRegister = m.output("DRegister", d_reg.out);
        let ARegister = m.output("ARegister", a_reg.out);

        Self {
            m,
            inM,
            instruction,
            reset,
            outM,
            writeM,
            addressM,
            pc,
            DRegister,
            ARegister,
        }
    }
}
