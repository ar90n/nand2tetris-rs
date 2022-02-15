#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_BasicLoop() {
        let vm_program = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/ProgramFlow/BasicLoop/BasicLoop.vm

// Computes the sum 1 + 2 + ... + argument[0] and pushes the 
// result onto the stack. Argument[0] is initialized by the test 
// script before this code starts running.
push constant 0    
pop local 0         // initializes sum = 0
label LOOP_START
push argument 0    
push local 0
add
pop local 0	        // sum = sum + counter
push argument 0
push constant 1
sub
pop argument 0      // counter--
push argument 0
if-goto LOOP_START  // If counter != 0, goto LOOP_START
push local 0

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
        m.write_memory(400, 3);
        m.prop();

        for _ in 0..600 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(256), 6);
    }
}
