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
// File name: projects/07/MemoryAccess/StaticTest/StaticTest.vm

// Executes pop and push commands using the static segment.
push constant 111
push constant 333
push constant 888
pop static 8
pop static 3
pop static 1
push static 3
push static 1
sub
push static 8
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

        assert_eq!(m.read_memory(256), 1110);
    }
}
