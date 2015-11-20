use std::io::{BufReader, BufWriter};
use std::collections::HashMap;
use std::char;

fn encode<T>(input: &mut T) -> Vec<u8> {
    //
    // T must implement the Read trait
    //
    let mut codes = HashMap::new();
    for val in 0..256 {
        let key = char::from_u32(val).expect("Character Error.").to_string();
        codes.insert(key, val);
    }
    let mut output = Vec::new();
    loop {
        let mut buf = [0; 1];
        let bytes_read = input.read(&mut buf).ok().expect("Bad read.");
        output.push(codes.entry(buf));
        if !bytes_read {
            break;
        }
    }
    output
}

fn main() {
    encode();
}

