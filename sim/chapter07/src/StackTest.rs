#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_StackTest() {
        let vm_program = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/07/StackArithmetic/StackTest/StackTest.vm

// Executes a sequence of arithmetic and logical operations
// on the stack. 
push constant 17
push constant 17
eq
push constant 17
push constant 16
eq
push constant 16
push constant 17
eq
push constant 892
push constant 891
lt
push constant 891
push constant 892
lt
push constant 891
push constant 891
lt
push constant 32767
push constant 32766
gt
push constant 32766
push constant 32767
gt
push constant 32766
push constant 32766
gt
push constant 57
push constant 31
push constant 53
add
push constant 112
sub
neg
and
push constant 82
or
not

"#
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let program = parse(&vm_program)
            .and_then(|p| translate(&p))
            .and_then(|p| assemble(&p))
            .unwrap();

   let mut m = Computer::new(program);
        m.reset = false;
        m.write_memory(0, 256);
        m.prop();

        for _ in 0..1000 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(0), 266);
        assert_eq!(m.read_memory(256), 0xffff);
        assert_eq!(m.read_memory(257), 0);
        assert_eq!(m.read_memory(258), 0);
        assert_eq!(m.read_memory(259), 0);
        assert_eq!(m.read_memory(260), 0xffff);
        assert_eq!(m.read_memory(261), 0);
        assert_eq!(m.read_memory(262), 0xffff);
        assert_eq!(m.read_memory(263), 0);
        assert_eq!(m.read_memory(264), 0);
        assert_eq!(m.read_memory(265), (0x10000 - 91));
    }
}
