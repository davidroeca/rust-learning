use std::io::{Read, Write};
use std::collections::HashMap;
use std::char;
use std::str::FromStr;
use std::u32;
use std::fmt;
//
// This code references: http://marknelson.us/2011/11/08/lzw-revisited/
// It attempts to implement this algorithm in a rusty manner.
//

// ------------------------------------------------------------------
// LZW driver functions
// ------------------------------------------------------------------
fn lzw_compress<T: Read, U: Write>(input: &mut T,
                                   output: &mut U,
                                   max_code: u32) {
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
    for c in input.chars() {
        let current_char = c.unwrap();
        // figure out a better numeric write
        current_string.push(current_char);
        match codes.get(&current_string) {
            None => {
                if (next_code <= max_code) {
                    codes.insert(current_string.clone(), next_code);
                    next_code += 1;
                }
                // Get code for string of known value; write
                current_string.pop();
                // WRITE
                output.write(format!("{}", codes.get(&current_string).expect("error")).as_ref());

                // Reset string to current character, continue search
                current_string.clear();
                current_string.push(current_char);
            },
            Some(_)=> ()
        };        
    }
    //print!("{}", codes.get(&current_string).unwrap())
    // WRITE
    output.write(format!("{}", codes.get(&current_string).expect("error")).as_ref());
}

fn lzw_decompress<T: Read, U: Write>(input: &mut T,
                                     output: &mut U,
                                     max_code: u32) {
    // Initialize the string dictionary
    let mut strings: HashMap<u32, String> = HashMap::new();
    for key in 0..256 {
        let val = char::from_u32(key).expect("Character Error.").to_string();
        strings.insert(key, val);
    }

    // define the mutable loop variables
    let mut next_code = 257;
    let mut previous_string = String::new();
    // Decompress
    for c in input.chars() {
        let current_char = c.unwrap();
        // debugging
        if current_char == ',' || current_char == ' ' {
            continue;
        }
        // figure out a better numeric read
        let current_code = u32::from_str(current_char.to_string().as_ref()).unwrap();
        match strings.get(&current_code) {
            None => {
                let prev_start = previous_string.chars()
                    .nth(0).unwrap().to_string();
                strings.insert(current_code, previous_string.clone() + &prev_start);
            },
            Some(_) => ()
        };
        let current_string = strings.get(&current_code).expect("error").clone();
        // WRITE
        output.write(format!("{}", current_string).as_ref());
        if !previous_string.is_empty() && next_code <= max_code {
            let current_start = current_string.chars()
                .nth(0).unwrap().to_string();
            strings.insert(next_code, previous_string.clone() + &current_start);
        }
        previous_string = current_string.clone();
    }
}

fn main() {
    lzw_compress(&mut std::io::stdin(), &mut std::io::stdout(), 12183);
}
