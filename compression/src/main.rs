extern crate bincode;
use bincode::SizeLimit;
use bincode::serde::{deserialize, serialize};
use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;
use std::char;
use std::str::FromStr;
use std::u32;
//
// This code references: http://marknelson.us/2011/11/08/lzw-revisited/
// It attempts to implement this algorithm in a rusty manner.
//fn write_binary<T: Write>(output: &mut T, s: String) {

// ------------------------------------------------------------------
// LZW driver functions
// ------------------------------------------------------------------
fn lzw_compress<T: Read, U: Write>(input: &mut T,
                                   output: &mut U,
                                   max_code: u32) {
    // Initialize the code dictionary
    let mut codes: HashMap<String, u32> = HashMap::new();
    for val in 0..256 {
        let key = (val as u8 as char).to_string();
        codes.insert(key, val);
    }

    // define the mutable loop variables
    let mut next_code = 257;
    let mut current_string = String::new();

    // Compress
    for byte in input.bytes() {
        let current_byte = byte.unwrap();
        let current_as_char = current_byte as char;
        // figure out a better numeric write
        current_string.push(current_as_char);

        match codes.get(&current_string) {
            None => {
                if next_code <= max_code {
                    codes.insert(current_string.clone(), next_code);
                    next_code += 1;
                }
                // Get code for string of known value; write
                current_string.pop();
                // WRITE
                output.write(serialize(codes.get(&current_string)
                                     .expect("Retrieval error"),
                                     SizeLimit::Infinite)
                             .expect("Serialize error")
                             .as_ref()).expect("Write error");
                // Reset string to current character, continue search
                current_string.clear();
                current_string.push(current_as_char);
            },
            Some(_)=> ()
        };        
    }
    //print!("{}", codes.get(&current_string).unwrap())
    // WRITE
    output.write(serialize(codes.get(&current_string)
                           .expect("Retrieval error"),
                           SizeLimit::Infinite)
                 .expect("Serialize error")
                 .as_ref()).expect("Write error");
    output.flush();
}

fn lzw_decompress<T: Read, U: Write>(input: &mut T,
                                     output: &mut U,
                                     max_code: u32) {
    // Initialize the string dictionary
    let mut strings: HashMap<u32, String> = HashMap::new();
    for key in 0..256 {
        let val = (key as u8 as char).to_string();
        strings.insert(key, val);
    }
    // define the mutable loop variables
    let mut next_code = 257;
    let mut previous_string = String::new();
    // Decompress
    for byte in input.bytes() {
        let current_byte = byte.unwrap();
        let current_serialized = (current_byte as char).to_string();
        let current_deserialized: String = deserialize(current_serialized.as_ref())
            .expect("Deserialization error");
        let current_code = u32::from_str(current_deserialized.as_ref()).expect("Conversion error");
        match strings.get(&current_code) {
            None => {
                let prev_start = previous_string.chars()
                    .nth(0).unwrap().to_string();
                strings.insert(current_code,
                               previous_string.clone() + &prev_start);
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
    output.flush();
}

fn main() {
    lzw_compress(&mut std::io::stdin(), &mut std::io::stdout(), 12183);
}
