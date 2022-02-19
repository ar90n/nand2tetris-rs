use compiler::tokenizer::{tokenize, dump_as_xml};

#[test]
fn test_square_squaregamet() {
	let jack_code = r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/10/Square/SquareGame.jack

// (same as projects/09/Square/SquareGame.jack)

/**
 * Implements the Square Dance game.
 * This simple game allows the user to move a black square around
 * the screen, and change the square's size during the movement.
 * When the game starts, a square of 30 by 30 pixels is shown at the
 * top-left corner of the screen. The user controls the square as follows.
 * The 4 arrow keys are used to move the square up, down, left, and right.
 * The 'z' and 'x' keys are used, respectively, to decrement and increment
 * the square's size. The 'q' key is used to quit the game.
 */

class SquareGame {
   field Square square; // the square of this game
   field int direction; // the square's current direction: 
                        // 0=none, 1=up, 2=down, 3=left, 4=right

   /** Constructs a new Square Game. */
   constructor SquareGame new() {
      // Creates a 30 by 30 pixels square and positions it at the top-left
      // of the screen.
      let square = Square.new(0, 0, 30);
      let direction = 0;  // initial state is no movement
      return this;
   }

   /** Disposes this game. */
   method void dispose() {
      do square.dispose();
      do Memory.deAlloc(this);
      return;
   }

   /** Moves the square in the current direction. */
   method void moveSquare() {
      if (direction = 1) { do square.moveUp(); }
      if (direction = 2) { do square.moveDown(); }
      if (direction = 3) { do square.moveLeft(); }
      if (direction = 4) { do square.moveRight(); }
      do Sys.wait(5);  // delays the next movement
      return;
   }

   /** Runs the game: handles the user's inputs and moves the square accordingly */
   method void run() {
      var char key;  // the key currently pressed by the user
      var boolean exit;
      let exit = false;
      
      while (~exit) {
         // waits for a key to be pressed
         while (key = 0) {
            let key = Keyboard.keyPressed();
            do moveSquare();
         }
         if (key = 81)  { let exit = true; }     // q key
         if (key = 90)  { do square.decSize(); } // z key
         if (key = 88)  { do square.incSize(); } // x key
         if (key = 131) { let direction = 1; }   // up arrow
         if (key = 133) { let direction = 2; }   // down arrow
         if (key = 130) { let direction = 3; }   // left arrow
         if (key = 132) { let direction = 4; }   // right arrow

         // waits for the key to be released
         while (~(key = 0)) {
            let key = Keyboard.keyPressed();
            do moveSquare();
         }
     } // while
     return;
   }
}"#;
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
<identifier> Square </identifier>
<symbol> . </symbol>
<identifier> new </identifier>
<symbol> ( </symbol>
<integerConstant> 0 </integerConstant>
<symbol> , </symbol>
<integerConstant> 0 </integerConstant>
<symbol> , </symbol>
<integerConstant> 30 </integerConstant>
<symbol> ) </symbol>
<symbol> ; </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<integerConstant> 0 </integerConstant>
<symbol> ; </symbol>
<keyword> return </keyword>
<keyword> this </keyword>
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
<keyword> this </keyword>
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
<symbol> = </symbol>
<integerConstant> 1 </integerConstant>
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
<symbol> = </symbol>
<integerConstant> 2 </integerConstant>
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
<symbol> = </symbol>
<integerConstant> 3 </integerConstant>
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
<symbol> = </symbol>
<integerConstant> 4 </integerConstant>
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
<integerConstant> 5 </integerConstant>
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
<keyword> false </keyword>
<symbol> ; </symbol>
<keyword> while </keyword>
<symbol> ( </symbol>
<symbol> ~ </symbol>
<identifier> exit </identifier>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> while </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> = </symbol>
<integerConstant> 0 </integerConstant>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> key </identifier>
<symbol> = </symbol>
<identifier> Keyboard </identifier>
<symbol> . </symbol>
<identifier> keyPressed </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
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
<symbol> = </symbol>
<integerConstant> 81 </integerConstant>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> exit </identifier>
<symbol> = </symbol>
<keyword> true </keyword>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> = </symbol>
<integerConstant> 90 </integerConstant>
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
<symbol> = </symbol>
<integerConstant> 88 </integerConstant>
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
<symbol> = </symbol>
<integerConstant> 131 </integerConstant>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<integerConstant> 1 </integerConstant>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> = </symbol>
<integerConstant> 133 </integerConstant>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<integerConstant> 2 </integerConstant>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> = </symbol>
<integerConstant> 130 </integerConstant>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<integerConstant> 3 </integerConstant>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> if </keyword>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> = </symbol>
<integerConstant> 132 </integerConstant>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> direction </identifier>
<symbol> = </symbol>
<integerConstant> 4 </integerConstant>
<symbol> ; </symbol>
<symbol> } </symbol>
<keyword> while </keyword>
<symbol> ( </symbol>
<symbol> ~ </symbol>
<symbol> ( </symbol>
<identifier> key </identifier>
<symbol> = </symbol>
<integerConstant> 0 </integerConstant>
<symbol> ) </symbol>
<symbol> ) </symbol>
<symbol> { </symbol>
<keyword> let </keyword>
<identifier> key </identifier>
<symbol> = </symbol>
<identifier> Keyboard </identifier>
<symbol> . </symbol>
<identifier> keyPressed </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
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
