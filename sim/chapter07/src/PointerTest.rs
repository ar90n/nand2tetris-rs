#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_PointerTest() {
        let vm_program = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/07/MemoryAccess/PointerTest/PointerTest.vm

// Executes pop and push commands using the 
// pointer, this, and that segments.
push constant 3030
pop pointer 0
push constant 3040
pop pointer 1
push constant 32
pop this 2
push constant 46
pop that 6
push pointer 0
push pointer 1
add
push this 2
sub
push that 6
add

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

        for _ in 0..600 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(256), 6084);
        assert_eq!(m.read_memory(3), 3030);
        assert_eq!(m.read_memory(4), 3040);
        assert_eq!(m.read_memory(3032), 32);
        assert_eq!(m.read_memory(3046), 46);
    }
}
