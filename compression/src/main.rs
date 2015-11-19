use std::io::{BufReader, BufWriter};
use std::collections::HashMap;

fn encode<T>(input: &mut T) -> Vec<u8> {
    //
    // T must implement the Read trait
    //
    let mut codes = HashMap::new();
    let mut output = Vec::new();
    loop {
        let mut buf = [0; 1]
        let bytes_read = input.read(&mut buf).ok().expect("Bad read.");
        output.push(codes.entry(buf));
        if !bytes_read {
            break;
        }
    }
    output
}

fn main() {


