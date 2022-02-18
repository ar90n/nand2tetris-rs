use assembler::code::{assemble, parse};

#[test]
fn test_add() {
    let asm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

@2
D=A
@3
D=D+A
@0
M=D
"#;

    let hack = vec![
        0b0000000000000010u16,
        0b1110110000010000u16,
        0b0000000000000011u16,
        0b1110000010010000u16,
        0b0000000000000000u16,
        0b1110001100001000u16,
    ];

    let lines = asm.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let commands = parse(&lines).unwrap();
    let binary = assemble(&commands).unwrap();

    assert_eq!(hack, binary);
}
