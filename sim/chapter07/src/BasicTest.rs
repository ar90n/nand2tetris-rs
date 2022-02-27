#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_BasicTest() {
        let vm_program = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/07/MemoryAccess/BasicTest/BasicTest.vm

// Executes pop and push commands using the virtual memory segments.
push constant 10
pop local 0
push constant 21
push constant 22
pop argument 2
pop argument 1
push constant 36
pop this 6
push constant 42
push constant 45
pop that 5
pop that 2
push constant 510
pop temp 6
push local 0
push that 5
add
push argument 1
sub
push this 6
push this 6
add
sub
push temp 6
add

"#
        .split("\n")
        .collect::<Vec<_>>();
        let program = parse(&vm_program)
            .and_then(|p| translate(&p, None))
            .and_then(|p| assemble(&p))
            .unwrap();

        let mut m = Computer::new(program);
        m.reset = false;
        m.write_memory(0, 256);
        m.write_memory(1, 300);
        m.write_memory(2, 400);
        m.write_memory(3, 3000);
        m.write_memory(4, 3010);
        m.prop();

        for _ in 0..600 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(256), 472);
        assert_eq!(m.read_memory(300), 10);
        assert_eq!(m.read_memory(401), 21);
        assert_eq!(m.read_memory(402), 22);
        assert_eq!(m.read_memory(3006), 36);
        assert_eq!(m.read_memory(3012), 42);
        assert_eq!(m.read_memory(3015), 45);
        assert_eq!(m.read_memory(11), 510);
    }
}
