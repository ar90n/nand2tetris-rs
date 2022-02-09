#[cfg(test)]
mod tests {
    use super::super::Computer::Computer;
    #[test]
    fn test_ComputerMax() {
        let expect_a_reg = vec![
            0, 0, 0, 1, 1, 10, 10, 1, 1, 12, 12, 2, 2, 14, 14, 14, 14, 0, 0, 1, 1, 10, 10, 0, 0, 2,
            2,
        ];
        let expect_d_reg = vec![
            0,
            0,
            3,
            3,
            (65536 - 2),
            (65536 - 2),
            (65536 - 2),
            (65536 - 2),
            5,
            5,
            5,
            5,
            5,
            5,
            5,
            5,
            5,
            5,
            23456,
            23456,
            11111,
            11111,
            11111,
            11111,
            23456,
            23456,
            23456,
        ];

        let expect_ram = vec![
            vec![
                3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 23456, 23456, 23456, 23456, 23456,
                23456, 23456, 23456, 23456, 23456, 23456,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 12345, 12345, 12345, 12345, 12345,
                12345, 12345, 12345, 12345, 12345, 12345,
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 23456,
            ],
        ];

        let program = vec![
            0b0000000000000000,
            0b1111110000010000,
            0b0000000000000001,
            0b1111010011010000,
            0b0000000000001010,
            0b1110001100000001,
            0b0000000000000001,
            0b1111110000010000,
            0b0000000000001100,
            0b1110101010000111,
            0b0000000000000000,
            0b1111110000010000,
            0b0000000000000010,
            0b1110001100001000,
            0b0000000000001110,
            0b1110101010000111,
        ];
        let mut m = Computer::new(program);
        let mut time = 0;

        let check_output = |m: &mut Computer, time: usize|{
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
        m.write_memory(0u32, 3u32);
        m.write_memory(1u32, 5u32);
        m.prop();
        check_output(&mut m, time);
        time += 1;
    
        for _ in 0..14 {
            step_next(&mut m);
            check_output(&mut m, time);
            time += 1;
        }

        m.reset = true;
        step_next(&mut m);
        check_output(&mut m, time);
        time += 1;

        m.reset = false;
        m.write_memory(0u32, 23456u32);
        m.write_memory(1u32, 12345u32);
        m.prop();
        check_output(&mut m, time);
        time += 1;

        for _ in 0..10 {
            step_next(&mut m);
            check_output(&mut m, time);
            time += 1;
        }
    }
}
