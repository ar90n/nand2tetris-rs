use super::compile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_bin() {
        let main = r#"
class Main {
    
    /**
     * Initializes RAM[8001]..RAM[8016] to -1,
     * and converts the value in RAM[8000] to binary.
     */
    function void main() {
	    var int value;
        do Main.fillMemory(8001, 16, -1); // sets RAM[8001]..RAM[8016] to -1
        let value = Memory.peek(8000);    // reads a value from RAM[8000]
        do Main.convert(value);           // performs the conversion
        return;
    }
    
    /** Converts the given decimal value to binary, and puts 
     *  the resulting bits in RAM[8001]..RAM[8016]. */
    function void convert(int value) {
    	var int mask, position;
    	var boolean loop;
    	
    	let loop = true;
    	while (loop) {
    	    let position = position + 1;
    	    let mask = Main.nextMask(mask);
    	
    	    if (~(position > 16)) {
    	
    	        if (~((value & mask) = 0)) {
    	            do Memory.poke(8000 + position, 1);
       	        }
    	        else {
    	            do Memory.poke(8000 + position, 0);
      	        }    
    	    }
    	    else {
    	        let loop = false;
    	    }
    	}
    	return;
    }
 
    /** Returns the next mask (the mask that should follow the given mask). */
    function int nextMask(int mask) {
    	if (mask = 0) {
    	    return 1;
    	}
    	else {
	    return mask * 2;
    	}
    }
    
    /** Fills 'length' consecutive memory locations with 'value',
      * starting at 'startAddress'. */
    function void fillMemory(int startAddress, int length, int value) {
        while (length > 0) {
            do Memory.poke(startAddress, value);
            let length = length - 1;
            let startAddress = startAddress + 1;
        }
        return;
    }
}"#;

        compile(main, "ConvertToBin", "Main".to_string());
    }
}
