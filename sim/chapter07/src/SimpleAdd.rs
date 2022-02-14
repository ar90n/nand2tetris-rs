#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_SimpleAdd() {
        let vm_program = vec![
            "push constant 7".to_string(),
            "push constant 8".to_string(),
            "add".to_string(),
        ];
        let vm_program = r#"push constant 7
push constant 8
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

        for _ in 0..60 {
            m.prop();
            m.posedge_clk();
            m.prop();
        }

        assert_eq!(m.read_memory(0), 257);
        assert_eq!(m.read_memory(256), 15);
    }
}
