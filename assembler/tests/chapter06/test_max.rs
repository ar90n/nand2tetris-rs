use assembler::code::{assemble, parse};

#[test]
fn test_max() {
    let asm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/max/Max.asm

// Computes R2 = max(R0, R1)  (R0,R1,R2 refer to RAM[0],RAM[1],RAM[2])

   @R0
   D=M              // D = first number
   @R1
   D=D-M            // D = first number - second number
   @OUTPUT_FIRST
   D;JGT            // if D>0 (first is greater) goto output_first
   @R1
   D=M              // D = second number
   @OUTPUT_D
   0;JMP            // goto output_d
(OUTPUT_FIRST)
   @R0             
   D=M              // D = first number
(OUTPUT_D)
   @R2
   M=D              // M[2] = D (greatest number)
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP            // infinite loop

"#;

    let hack = vec![
        0b0000000000000000u16,
        0b1111110000010000u16,
        0b0000000000000001u16,
        0b1111010011010000u16,
        0b0000000000001010u16,
        0b1110001100000001u16,
        0b0000000000000001u16,
        0b1111110000010000u16,
        0b0000000000001100u16,
        0b1110101010000111u16,
        0b0000000000000000u16,
        0b1111110000010000u16,
        0b0000000000000010u16,
        0b1110001100001000u16,
        0b0000000000001110u16,
        0b1110101010000111u16,
    ];

    let lines = asm.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let commands = parse(&lines).unwrap();
    let binary = assemble(&commands).unwrap();

    assert_eq!(hack, binary);
}
