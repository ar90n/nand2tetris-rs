#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{bootstrap, parse, translate};

    #[test]
    fn test_StaticsTest() {
        let sys_vm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/StaticsTest/Sys.vm

// Tests that different functions, stored in two different 
// class files, manipulate the static segment correctly. 
function Sys.init 0
push constant 6
push constant 8
call Class1.set 2
pop temp 0 // Dumps the return value
push constant 23
push constant 15
call Class2.set 2
pop temp 0 // Dumps the return value
call Class1.get 0
call Class2.get 0
label WHILE
goto WHILE

"#
        .split("\n")
        .collect::<Vec<_>>();

        let class1_vm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/StaticsTest/Class1.vm

// Stores two supplied arguments in static[0] and static[1].
function Class1.set 0
push argument 0
pop static 0
push argument 1
pop static 1
push constant 0
return

// Returns static[0] - static[1].
function Class1.get 0
push static 0
push static 1
sub
return

"#
        .split("\n")
        .collect::<Vec<_>>();

        let class2_vm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/StaticsTest/Class2.vm

// Stores two supplied arguments in static[0] and static[1].
function Class2.set 0
push argument 0
pop static 0
push argument 1
pop static 1
push constant 0
return

// Returns static[0] - static[1].
function Class2.get 0
push static 0
push static 1
sub
return

"#
        .split("\n")
        .collect::<Vec<_>>();

        let sys_vm_commands = parse(&sys_vm)
            .and_then(|p| translate(&p, Some("sys")))
            .unwrap();
        let class1_vm_commands = parse(&class1_vm).and_then(|p| translate(&p, Some("class1"))).unwrap();
        let class2_vm_commands = parse(&class2_vm).and_then(|p| translate(&p, Some("class2"))).unwrap();
        let mut vm_program = bootstrap(261);
        vm_program.extend(sys_vm_commands);
        vm_program.extend(class1_vm_commands);
        vm_program.extend(class2_vm_commands);

        let program = assemble(&vm_program).unwrap();

        let mut m = Computer::new(program);
        m.reset = false;
        m.prop();

        for _ in 0..2500 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(0), 263);
        assert_eq!(m.read_memory(261), (65536 - 2));
        assert_eq!(m.read_memory(262), 8);
    }
}
