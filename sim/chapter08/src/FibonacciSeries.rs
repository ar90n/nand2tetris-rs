#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_FibonacciSeries() {
        let vm_program = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/ProgramFlow/FibonacciSeries/FibonacciSeries.vm

// Puts the first argument[0] elements of the Fibonacci series
// in the memory, starting in the address given in argument[1].
// Argument[0] and argument[1] are initialized by the test script 
// before this code starts running.

push argument 1
pop pointer 1           // that = argument[1]

push constant 0
pop that 0              // first element in the series = 0
push constant 1
pop that 1              // second element in the series = 1

push argument 0
push constant 2
sub
pop argument 0          // num_of_elements -= 2 (first 2 elements are set)

label MAIN_LOOP_START

push argument 0
if-goto COMPUTE_ELEMENT // if num_of_elements > 0, goto COMPUTE_ELEMENT
goto END_PROGRAM        // otherwise, goto END_PROGRAM

label COMPUTE_ELEMENT

push that 0
push that 1
add
pop that 2              // that[2] = that[0] + that[1]

push pointer 1
push constant 1
add
pop pointer 1           // that += 1

push argument 0
push constant 1
sub
pop argument 0          // num_of_elements--

goto MAIN_LOOP_START

label END_PROGRAM

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
        m.write_memory(400, 6);
        m.write_memory(401, 3000);
        m.prop();

        for _ in 0..1100 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(3000), 0);
        assert_eq!(m.read_memory(3001), 1);
        assert_eq!(m.read_memory(3002), 1);
        assert_eq!(m.read_memory(3003), 2);
        assert_eq!(m.read_memory(3004), 3);
        assert_eq!(m.read_memory(3005), 5);
    }
}
