use compiler::tokenizer::{tokenize, dump_as_xml};

#[test]
fn test_expression_less_square_squaret() {
	let jack_code = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
/// File name: projects/10/ExpressionLessSquare/Square.jack

/** Expressionless version of projects/10/Square/Square.jack. */

class Square {

   field int x, y; 
   field int size; 

   constructor Square new(int Ax, int Ay, int Asize) {
      let x = Ax;
      let y = Ay;
      let size = Asize;
      do draw();
      return x;
   }

   method void dispose() {
      do Memory.deAlloc(this);
      return;
   }

   method void draw() {
      do Screen.setColor(x);
      do Screen.drawRectangle(x, y, x, y);
      return;
   }

   method void erase() {
      do Screen.setColor(x);
      do Screen.drawRectangle(x, y, x, y);
      return;
   }

   method void incSize() {
      if (x) {
         do erase();
         let size = size;
         do draw();
      }
      return;
   }

   method void decSize() {
      if (size) {
         do erase();
         let size = size;
         do draw();
      }
      return;
   }

   method void moveUp() {
      if (y) {
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
         let y = y;
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
      }
      return;
   }

   method void moveDown() {
      if (y) {
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
         let y = y;
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
      }
      return;
   }

   method void moveLeft() {
      if (x) {
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
         let x = x;
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
      }
      return;
   }

   method void moveRight() {
      if (x) {
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
         let x = x;
         do Screen.setColor(x);
         do Screen.drawRectangle(x, y, x, y);
      }
      return;
   }
}"#;

	let tags = r#"<tokens>
<keyword> class </keyword>
<identifier> Square </identifier>
<symbol> { </symbol>
<keyword> field </keyword>
<keyword> int </keyword>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ; </symbol>
<keyword> field </keyword>
<keyword> int </keyword>
<identifier> size </identifier>
<symbol> ; </symbol>
<keyword> constructor </keyword>
<identifier> Square </identifier>
<identifier> new </identifier>
<symbol> ( </symbol>
<keyword> int </keyword>
<identifier> Ax </identifier>
<symbol> , </symbol>
<keyword> int </keyword>
<identifier> Ay </identifier>
<symbol> , </symbol>
<keyword> int </keyword>
<identifier> Asize </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> x </identifier>
<symbol> = </symbol>
<identifier> Ax </identifier>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> y </identifier>
<symbol> = </symbol>
<identifier> Ay </identifier>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> size </identifier>
<symbol> = </symbol>
<identifier> Asize </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> draw </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> return </keyword>
<identifier> x </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> dispose </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Memory </identifier>
<symbol> . </symbol>
<identifier> deAlloc </identifier>
<symbol> ( </symbol>
<keyword> this </keyword>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> draw </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> erase </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> incSize </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> erase </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> size </identifier>
<symbol> = </symbol>
<identifier> size </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> draw </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> decSize </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> size </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> erase </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> size </identifier>
<symbol> = </symbol>
<identifier> size </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> draw </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> moveUp </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> y </identifier>
<symbol> = </symbol>
<identifier> y </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> moveDown </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> y </identifier>
<symbol> = </symbol>
<identifier> y </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> moveLeft </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> x </identifier>
<symbol> = </symbol>
<identifier> x </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> moveRight </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> x </identifier>
<symbol> = </symbol>
<identifier> x </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> setColor </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Screen </identifier>
<symbol> . </symbol>
<identifier> drawRectangle </identifier>
<symbol> ( </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> , </symbol>
<identifier> x </identifier>
<symbol> , </symbol>
<identifier> y </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<symbol> } </symbol>
</tokens>"#;

	let tokens = tokenize(jack_code).unwrap();
	let dumped = dump_as_xml(&tokens);
	//assert_eq!(dumped, tags);

  for (i, line) in dumped.lines().enumerate() {
    assert_eq!(tags.lines().nth(i).unwrap(), line);
  }
}
