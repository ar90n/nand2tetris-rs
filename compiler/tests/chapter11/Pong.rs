use super::compile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pong() {
        let main = r#"
class Main {

    /** Initializes a Pong game and starts running it. */
    function void main() {
        var PongGame game;
        do PongGame.newInstance();
        let game = PongGame.getInstance();
        do game.run();
        do game.dispose();
        return;
    }
}"#;

        let ball = r#"
class Ball {

    field int x, y;               // the ball's screen location (in pixels)
    field int lengthx, lengthy;   // distance of last destination (in pixels)

    field int d, straightD, diagonalD;            // used for straight line movement computation
    field boolean invert, positivex, positivey;   // (same)
   
    field int leftWall, rightWall, topWall, bottomWall;  // wall locations
   
    field int wall;   // last wall that the ball was bounced off of

    /** Constructs a new ball with the given initial location and wall locations. */
    constructor Ball new(int Ax, int Ay,
                         int AleftWall, int ArightWall, int AtopWall, int AbottomWall) {    	
	    let x = Ax;		
	    let y = Ay;
	    let leftWall = AleftWall;
	    let rightWall = ArightWall - 6;    // -6 for ball size
	    let topWall = AtopWall; 
	    let bottomWall = AbottomWall - 6;  // -6 for ball size
	    let wall = 0;
        do show();
        return this;
    }

    /** Deallocates the Ball's memory. */
    method void dispose() {
        do Memory.deAlloc(this);
        return;
    }

    /** Shows the ball. */
    method void show() {
        do Screen.setColor(true);
        do draw();
        return;
    }

    /** Hides the ball. */
    method void hide() {
        do Screen.setColor(false);
	    do draw();
        return;
    }

    /** Draws the ball. */
    method void draw() {
	    do Screen.drawRectangle(x, y, x + 5, y + 5);
	    return;
    }

    /** Returns the ball's left edge. */
    method int getLeft() {
        return x;
    }

    /** Returns the ball's right edge. */
    method int getRight() {
        return x + 5;
    }

    /** Computes and sets the ball's destination. */
    method void setDestination(int destx, int desty) {
        var int dx, dy, temp;
  	    let lengthx = destx - x;
	    let lengthy = desty - y;
        let dx = Math.abs(lengthx);
        let dy = Math.abs(lengthy);
        let invert = (dx < dy);

        if (invert) {
            let temp = dx; // swap dx, dy
            let dx = dy;
            let dy = temp;
   	        let positivex = (y < desty);
            let positivey = (x < destx);
        }
        else {
	        let positivex = (x < destx);
            let positivey = (y < desty);
        }

        let d = (2 * dy) - dx;
        let straightD = 2 * dy;
        let diagonalD = 2 * (dy - dx);

	    return;
    }

    /**
     * Moves the ball one unit towards its destination.
     * If the ball has reached a wall, returns 0.
     * Else, returns a value according to the wall:
     * 1 (left wall), 2 (right wall), 3 (top wall), 4 (bottom wall).
     */
    method int move() {

	    do hide();

        if (d < 0) { let d = d + straightD; }
        else {
            let d = d + diagonalD;

            if (positivey) {
                if (invert) { let x = x + 4; }
                else { let y = y + 4; }
            }
            else {
                if (invert) { let x = x - 4; }
                else { let y = y - 4; }
            }
	    }

        if (positivex) {
            if (invert) { let y = y + 4; }
            else { let x = x + 4; }
	    }
	    else {
            if (invert) { let y = y - 4; }
            else { let x = x - 4; }
	    }

	    if (~(x > leftWall)) {
	        let wall = 1;    
	        let x = leftWall;
	    }
        if (~(x < rightWall)) {
	        let wall = 2;    
	        let x = rightWall;
	    }
        if (~(y > topWall)) {
            let wall = 3;    
	        let y = topWall;
        }
        if (~(y < bottomWall)) {
            let wall = 4;    
	        let y = bottomWall;
        }

	    do show();

	    return wall;
    }

    /**
     * Bounces off the current wall: sets the new destination
     * of the ball according to the ball's angle and the given
     * bouncing direction (-1/0/1=left/center/right or up/center/down).
     */
    method void bounce(int bouncingDirection) {
        var int newx, newy, divLengthx, divLengthy, factor;

	    // dividing by 10 first since results are too big
        let divLengthx = lengthx / 10;
        let divLengthy = lengthy / 10;
	    if (bouncingDirection = 0) { let factor = 10; }
	    else {
	        if (((~(lengthx < 0)) & (bouncingDirection = 1)) | ((lengthx < 0) & (bouncingDirection = (-1)))) {
                let factor = 20; // bounce direction is in ball direction
            }
	        else { let factor = 5; } // bounce direction is against ball direction
	    }

	    if (wall = 1) {
	        let newx = 506;
	        let newy = (divLengthy * (-50)) / divLengthx;
            let newy = y + (newy * factor);
	    }
        else {
            if (wall = 2) {
                let newx = 0;
                let newy = (divLengthy * 50) / divLengthx;
                let newy = y + (newy * factor);
	        }
	        else {
                if (wall = 3) {
		            let newy = 250;
		            let newx = (divLengthx * (-25)) / divLengthy;
                    let newx = x + (newx * factor);
		        }
                else { // assumes wall = 4
		            let newy = 0;
		            let newx = (divLengthx * 25) / divLengthy;
                    let newx = x + (newx * factor);
		        }
            }
        }

        do setDestination(newx, newy);
        return;
    }
}"#;

        let bat = r#"
class Bat {

    field int x, y;           // the bat's screen location
    field int width, height;  // the bat's width and height
    field int direction;      // direction of the bat's movement (1 = left, 2 = right)

    /** Constructs a new bat with the given location and width. */
    constructor Bat new(int Ax, int Ay, int Awidth, int Aheight) {
        let x = Ax;
        let y = Ay;
        let width = Awidth;
        let height = Aheight;
        let direction = 2;
        do show();
        return this;
    }

    /** Deallocates the object's memory. */
    method void dispose() {
        do Memory.deAlloc(this);
        return;
    }

    /** Shows the bat. */
    method void show() {
        do Screen.setColor(true);
        do draw();
        return;
    }

    /** Hides the bat. */
    method void hide() {
        do Screen.setColor(false);
        do draw();
        return;
    }

    /** Draws the bat. */
    method void draw() {
        do Screen.drawRectangle(x, y, x + width, y + height);
        return;
    }

    /** Sets the bat's direction (0=stop, 1=left, 2=right). */
    method void setDirection(int Adirection) {
        let direction = Adirection;
        return;
    }

    /** Returns the bat's left edge. */
    method int getLeft() {
        return x;
    }

    /** Returns the bat's right edge. */
    method int getRight() {
        return x + width;
    }

    /** Sets the bat's width. */
    method void setWidth(int Awidth) {
        do hide();
        let width = Awidth;
        do show();
        return;
    }

    /** Moves the bat one step in the bat's direction. */
    method void move() {
	    if (direction = 1) {
            let x = x - 4;
            if (x < 0) { let x = 0; }
            do Screen.setColor(false);
            do Screen.drawRectangle((x + width) + 1, y, (x + width) + 4, y + height);
            do Screen.setColor(true);
            do Screen.drawRectangle(x, y, x + 3, y + height);
        }
        else {
            let x = x + 4;
            if ((x + width) > 511) { let x = 511 - width; }
            do Screen.setColor(false);
            do Screen.drawRectangle(x - 4, y, x - 1, y + height);
            do Screen.setColor(true);
            do Screen.drawRectangle((x + width) - 3, y, x + width, y + height);
        }
        return;
    }
}"#;

        let pong_game = r#"
class PongGame {

    static PongGame instance; // the singelton, a Pong game instance     
    field Bat bat;            // the bat
    field Ball ball;          // the ball
    field int wall;           // the current wall that the ball is bouncing off of.
    field boolean exit;       // true when the game is over
    field int score;          // the current score.
    field int lastWall;       // the last wall that the ball bounced off of.

    // The current width of the bat
    field int batWidth;

    /** Constructs a new Pong game. */
    constructor PongGame new() {
	    do Screen.clearScreen();
        let batWidth = 50;  // initial bat size
        let bat = Bat.new(230, 229, batWidth, 7);
        let ball = Ball.new(253, 222, 0, 511, 0, 229);
        do ball.setDestination(400,0);
        do Screen.drawRectangle(0, 238, 511, 240);
	    do Output.moveCursor(22,0);
	    do Output.printString("Score: 0");
	
	    let exit = false;
	    let score = 0;
	    let wall = 0;
	    let lastWall = 0;

        return this;
    }

    /** Deallocates the object's memory. */
    method void dispose() {
        do bat.dispose();
	    do ball.dispose();
        do Memory.deAlloc(this);
        return;
    }

    /** Creates an instance of Pong game, and stores it. */
    function void newInstance() {
        let instance = PongGame.new();
        return;
    }
    
    /** Returns the single instance of this Pong game. */
    function PongGame getInstance() {
        return instance;
    }

    /** Starts the game, and andles inputs from the user that control
     *  the bat's movement direction. */
    method void run() {
        var char key;

        while (~exit) {
            // waits for a key to be pressed.
            while ((key = 0) & (~exit)) {
                let key = Keyboard.keyPressed();
                do bat.move();
                do moveBall();
                do Sys.wait(50);
            }

            if (key = 130) { do bat.setDirection(1); }
	        else {
	            if (key = 132) { do bat.setDirection(2); }
		        else {
                    if (key = 140) { let exit = true; }
		        }
            }

            // Waits for the key to be released.
            while ((~(key = 0)) & (~exit)) {
                let key = Keyboard.keyPressed();
                do bat.move();
                do moveBall();
                do Sys.wait(50);
            }
        }

	    if (exit) {
            do Output.moveCursor(10,27);
	        do Output.printString("Game Over");
	    }
            
        return;
    }

    /**
     * Handles ball movement, including bouncing.
     * If the ball bounces off a wall, finds its new direction.
     * If the ball bounces off the bat, increases the score by one
     * and shrinks the bat's size, to make the game more challenging. 
     */
    method void moveBall() {
        var int bouncingDirection, batLeft, batRight, ballLeft, ballRight;

        let wall = ball.move();

        if ((wall > 0) & (~(wall = lastWall))) {
            let lastWall = wall;
            let bouncingDirection = 0;
            let batLeft = bat.getLeft();
            let batRight = bat.getRight();
            let ballLeft = ball.getLeft();
            let ballRight = ball.getRight();
  
            if (wall = 4) {
                let exit = (batLeft > ballRight) | (batRight < ballLeft);
                if (~exit) {
                    if (ballRight < (batLeft + 10)) { let bouncingDirection = -1; }
                    else {
                        if (ballLeft > (batRight - 10)) { let bouncingDirection = 1; }
                    }

                    let batWidth = batWidth - 2;
                    do bat.setWidth(batWidth);      
                    let score = score + 1;
                    do Output.moveCursor(22,7);
                    do Output.printInt(score);
                }
            }
            do ball.bounce(bouncingDirection);
        }
        return;
    }
}"#;

        compile(main, "Pong", "Main".to_string());
        compile(ball, "Pong", "Ball".to_string());
        compile(bat, "Pong", "Bat".to_string());
        compile(pong_game, "Pong", "PongGame".to_string());
    }
}
