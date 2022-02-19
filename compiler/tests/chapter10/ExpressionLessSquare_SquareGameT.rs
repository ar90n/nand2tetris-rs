use compiler::tokenizer::{tokenize, dump_as_xml};

#[test]
fn test_expression_less_square_squaregamet() {
	let jack_code = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/10/ExpressionLessSquare/SquareGame.jack

/** Expressionless version of projects/10/Square/SquareGame.jack. */

class SquareGame {
   field Square square; 
   field int direction; 

   constructor SquareGame new() {
      let square = square;
      let direction = direction;
      return square;
   }

   method void dispose() {
      do square.dispose();
      do Memory.deAlloc(square);
      return;
   }

   method void moveSquare() {
      if (direction) { do square.moveUp(); }
      if (direction) { do square.moveDown(); }
      if (direction) { do square.moveLeft(); }
      if (direction) { do square.moveRight(); }
      do Sys.wait(direction);
      return;
   }

   method void run() {
      var char key;
      var boolean exit;
      
      let exit = key;
      while (exit) {
         while (key) {
            let key = key;
            do moveSquare();
         }

         if (key) { let exit = exit; }
         if (key) { do square.decSize(); }
         if (key) { do square.incSize(); }
         if (key) { let direction = exit; }
         if (key) { let direction = key; }
         if (key) { let direction = square; }
         if (key) { let direction = direction; }

         while (key) {
            let key = key;
            do moveSquare();
         }
      }
      return;
    }
}
"#;
	let tags = r#"<tokens>
<keyword> class </keyword>
<identifier> SquareGame </identifier>
<symbol> { </symbol>
<keyword> field </keyword>
<identifier> Square </identifier>
<identifier> square </identifier>
<symbol> ; </symbol>
<keyword> field </keyword>
<keyword> int </keyword>
<identifier> direction </identifier>
<symbol> ; </symbol>
<keyword> constructor </keyword>
<identifier> SquareGame </identifier>
<identifier> new </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> square </identifier>
<symbol> = </symbol>
<identifier> square </identifier>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<identifier> direction </identifier>
<symbol> ; </symbol>
<keyword> return </keyword>
<identifier> square </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> dispose </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> dispose </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> Memory </identifier>
<symbol> . </symbol>
<identifier> deAlloc </identifier>
<symbol> ( </symbol>
<identifier> square </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> moveSquare </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> direction </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> moveUp </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> direction </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> moveDown </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> direction </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> moveLeft </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> direction </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> moveRight </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> do </keyword>
<identifier> Sys </identifier>
<symbol> . </symbol>
<identifier> wait </identifier>
<symbol> ( </symbol>
<identifier> direction </identifier>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> method </keyword>
<keyword> void </keyword>
<identifier> run </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> var </keyword>
<keyword> char </keyword>
<identifier> key </identifier>
<symbol> ; </symbol>
<keyword> var </keyword>
<keyword> boolean </keyword>
<identifier> exit </identifier>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> exit </identifier>
<symbol> = </symbol>
<identifier> key </identifier>
<symbol> ; </symbol>
<keyword> while </keyword>
<symbol> ( </symbol>
<identifier> exit </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> while </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> key </identifier>
<symbol> = </symbol>
<identifier> key </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> moveSquare </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> exit </identifier>
<symbol> = </symbol>
<identifier> exit </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> decSize </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> do </keyword>
<identifier> square </identifier>
<symbol> . </symbol>
<identifier> incSize </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<identifier> exit </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<identifier> key </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<identifier> square </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<identifier> direction </identifier>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> while </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> key </identifier>
<symbol> = </symbol>
<identifier> key </identifier>
<symbol> ; </symbol>
<keyword> do </keyword>
<identifier> moveSquare </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
<symbol> } </symbol>
<symbol> } </symbol>
<keyword> return </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<symbol> } </symbol>
</tokens>"#;

	let tokens = tokenize(jack_code).unwrap();
	let dumped = dump_as_xml(&tokens);
	assert_eq!(dumped, tags);
}
