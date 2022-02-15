#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_SimpleFunction() {
        let vm_program = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/SimpleFunction/SimpleFunction.vm

// Performs a simple calculation and returns the result.
function SimpleFunction.test 2
push local 0
push local 1
add
not
push argument 0
add
push argument 1
sub
return

"#
        .split("\n")
        .collect::<Vec<_>>();
        let program = parse(&vm_program)
            .and_then(|p| translate(&p, None))
            .and_then(|p| assemble(&p))
            .unwrap();

        let mut m = Computer::new(program);
        m.reset = false;
        m.write_memory(0, 317);
        m.write_memory(1, 317);
        m.write_memory(2, 310);
        m.write_memory(3, 3000);
        m.write_memory(4, 4000);
        m.write_memory(310, 1234);
        m.write_memory(311, 37);
        m.write_memory(312, 1000);
        m.write_memory(313, 305);
        m.write_memory(314, 300);
        m.write_memory(315, 3010);
        m.write_memory(316, 4010);
        m.prop();

        for _ in 0..300 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(0), 311);
        assert_eq!(m.read_memory(1), 305);
        assert_eq!(m.read_memory(2), 300);
        assert_eq!(m.read_memory(3), 3010);
        assert_eq!(m.read_memory(4), 4010);
        assert_eq!(m.read_memory(310), 1196);
    }
}
