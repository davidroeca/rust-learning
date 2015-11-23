use std::io::{Read, Write};
use std::collections::HashMap;
use std::char;
//
// This code references: http://marknelson.us/2011/11/08/lzw-revisited/
// It attempts to implement this algorithm in a rusty manner.
//

// ------------------------------------------------------------------
// LZW driver functions
// ------------------------------------------------------------------
fn lzw_compress<T: Read, U: Write>(input: &T, output: &mut U) {
    //
    // Type T must implement the Read trait
    // Type U must implement the Write trait
    //

    // Initialize the code dictionary
    let mut codes: HashMap<String, u32> = HashMap::new();
    for val in 0..256 {
        let key = char::from_u32(val).expect("Character Error.").to_string();
        codes.insert(key, val);
    }

    // define the mutable loop variables
    let mut next_code = 257;
    let mut current_string = String::new();

    // Compress
    loop {
        // Read the next byte, break from loop if nothing more to read
        let mut buf = [0; 1];
        let bytes_read = input.read(&mut buf).ok().expect("Bad read.");
        if !bytes_read {
            break;
        }
        // Add the byte to current string
        current_string.push(buf);
        match codes.get(current_string) {
            None => {
                codes.insert(current_string, next_code);
                next_code += 1;

                // Get code for string of known value; write
                current_string.pop();
                output.write("{}", codes.get(current_string).unwrap());

                // Reset string to current character, continue search
                current_string.clear();
                current_string.push(buf);
            },
            Some(_)=> ()
        };        
    }
    // Write final code
    output.write("{}", codes.get(current_string).unwrap());
}

fn lzw_decompress<T: Read, U: Write>(input: &T, output: &mut U) {
}

