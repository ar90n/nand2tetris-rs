#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    #[test]
    fn test_ComputerAdd() {
        let expect_a_reg = vec![0, 2, 2, 3, 3, 0, 0, 0, 2, 2, 3, 3, 0, 0];
        let expect_d_reg = vec![0, 0, 2, 2, 5, 5, 5, 5, 5, 2, 2, 5, 5, 5];
        let expect_ram = vec![
            vec![0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 5],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let program = vec![
            0b0000000000000010,
            0b1110110000010000,
            0b0000000000000011,
            0b1110000010010000,
            0b0000000000000000,
            0b1110001100001000,
        ];
        let mut m = Computer::new(program);
        let mut time = 0;

        let check_output = |m: &mut Computer, time: usize| {
            assert_eq!(m.read_a_reg(), expect_a_reg[time]);
            assert_eq!(m.read_d_reg(), expect_d_reg[time]);
            assert_eq!(m.read_memory(0), expect_ram[0][time]);
            assert_eq!(m.read_memory(1), expect_ram[1][time]);
            assert_eq!(m.read_memory(2), expect_ram[2][time]);
        };

        let step_next = |m: &mut Computer| {
            m.prop();
            m.posedge_clk();
            m.prop();
        };

        m.reset = false;
        m.prop();
        check_output(&mut m, time);
        time += 1;

        for _ in 0..6 {
            step_next(&mut m);
            check_output(&mut m, time);
            time += 1;
        }

        m.reset = true;
        m.write_memory(0, 0);

        step_next(&mut m);
        check_output(&mut m, time);
        time += 1;

        m.reset = false;

        for _ in 0..6 {
            step_next(&mut m);
            check_output(&mut m, time);
            time += 1;
        }
    }
}
