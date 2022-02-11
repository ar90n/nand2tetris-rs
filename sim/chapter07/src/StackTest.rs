#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    use assembler::code::assemble;
    use vm::code::{parse, translate};

    #[test]
    fn test_StackTest() {
        let vm_program = vec![
            "push constant 17".to_string(),
            "push constant 17".to_string(),
            "eq".to_string(),
            "push constant 17".to_string(),
            "push constant 16".to_string(),
            "eq".to_string(),
            "push constant 16".to_string(),
            "push constant 17".to_string(),
            "eq".to_string(),
            "push constant 892".to_string(),
            "push constant 891".to_string(),
            "lt".to_string(),
            "push constant 891".to_string(),
            "push constant 892".to_string(),
            "lt".to_string(),
            "push constant 891".to_string(),
            "push constant 891".to_string(),
            "lt".to_string(),
            "push constant 32767".to_string(),
            "push constant 32766".to_string(),
            "gt".to_string(),
            "push constant 32766".to_string(),
            "push constant 32767".to_string(),
            "gt".to_string(),
            "push constant 32766".to_string(),
            "push constant 32766".to_string(),
            "gt".to_string(),
            "push constant 57".to_string(),
            "push constant 31".to_string(),
            "push constant 53".to_string(),
            "add".to_string(),
            "push constant 112".to_string(),
            "sub".to_string(),
            "neg".to_string(),
            "and".to_string(),
            "push constant 82".to_string(),
            "or".to_string(),
            "not".to_string(),
        ];
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
