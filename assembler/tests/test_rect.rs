use assembler::code::{assemble, parse};

#[test]
fn test_rect() {
    let asm = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/rect/Rect.asm

// Draws a rectangle at the top-left corner of the screen.
// The rectangle is 16 pixels wide and R0 pixels high.

   @0
   D=M
   @INFINITE_LOOP
   D;JLE 
   @counter
   M=D
   @SCREEN
   D=A
   @address
   M=D
(LOOP)
   @address
   A=M
   M=-1
   @address
   D=M
   @32
   D=D+A
   @address
   M=D
   @counter
   MD=M-1
   @LOOP
   D;JGT
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP

"#;

    let hack = vec![
        0b0000000000000000u16,
        0b1111110000010000u16,
        0b0000000000010111u16,
        0b1110001100000110u16,
        0b0000000000010000u16,
        0b1110001100001000u16,
        0b0100000000000000u16,
        0b1110110000010000u16,
        0b0000000000010001u16,
        0b1110001100001000u16,
        0b0000000000010001u16,
        0b1111110000100000u16,
        0b1110111010001000u16,
        0b0000000000010001u16,
        0b1111110000010000u16,
        0b0000000000100000u16,
        0b1110000010010000u16,
        0b0000000000010001u16,
        0b1110001100001000u16,
        0b0000000000010000u16,
        0b1111110010011000u16,
        0b0000000000001010u16,
        0b1110001100000001u16,
        0b0000000000010111u16,
        0b1110101010000111u16,
    ];

    let lines = asm.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let commands = parse(&lines).unwrap();
    let binary = assemble(&commands).unwrap();

    assert_eq!(hack, binary);
}
