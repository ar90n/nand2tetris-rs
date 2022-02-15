#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{bootstrap, parse, translate};

    #[test]
    fn test_FibonacciElement() {
        let sys_vm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/FibonacciElement/Sys.vm

// Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
// function, which computes the n'th element of the Fibonacci series.
// Note that by convention, the Sys.init function is called "automatically" 
// by the bootstrap code.

function Sys.init 0
push constant 4
call Main.fibonacci 1   // computes the 4'th fibonacci element
label WHILE
goto WHILE              // loops infinitely

"#
        .split("\n")
        .collect::<Vec<_>>();

        let main_vm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/FibonacciElement/Main.vm

// Computes the n'th element of the Fibonacci series, recursively.
// n is given in argument[0].  Called by the Sys.init function 
// (part of the Sys.vm file), which also pushes the argument[0] 
// parameter before this code starts running.

function Main.fibonacci 0
push argument 0
push constant 2
lt                     // checks if n<2
if-goto IF_TRUE
goto IF_FALSE
label IF_TRUE          // if n<2, return n
push argument 0        
return
label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1)
push argument 0
push constant 2
sub
call Main.fibonacci 1  // computes fib(n-2)
push argument 0
push constant 1
sub
call Main.fibonacci 1  // computes fib(n-1)
add                    // returns fib(n-1) + fib(n-2)
return

"#
        .split("\n")
        .collect::<Vec<_>>();

        let sys_vm_commands = parse(&sys_vm).and_then(|p| translate(&p, None)).unwrap();
        let main_vm_commands = parse(&main_vm).and_then(|p| translate(&p, None)).unwrap();
        let mut vm_program = bootstrap(256);
        vm_program.extend(main_vm_commands);
        vm_program.extend(sys_vm_commands);

        let program = assemble(&vm_program).unwrap();

        let mut m = Computer::new(program);
        m.reset = false;
        m.prop();

        for _ in 0..6000 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(262), 3);
    }
}