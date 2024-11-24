
/*
    Just copy the contents of aoc_utils/src/main.rs into this.
    It is not cheating ;). 
 */

use std::io::BufRead;
use aoc_utils::aoc_utils::PuzzleInput;

fn main() {
    let p = match PuzzleInput::init(None) {
        Ok(x) => x,
        Err(e) => panic!("{e}")
    };

    println!("\n---\nReading file {:?} into a vector and printing it.", p.file_input);
    match p.vectorized() {
        Ok(v) => {
            v.iter().for_each(|x| println!("{:?}", x));
        },
        Err(e) => panic!("{e}")
    };

    println!("\n---\nReading file {:?} as a buffered input and printing each line.", p.file_input);
    match p.bufferized() {
        Ok(b) => {
            b.lines()
                .into_iter()
                .map(|x| x.unwrap())
                .for_each(|y| println!("{:?}", y));
        },
        Err(e) => panic!("{e}")
    };
}