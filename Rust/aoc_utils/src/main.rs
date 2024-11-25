
use std::io::BufRead;
use std::error::Error;
use aoc_utils::aoc_utils::PuzzleInput;

fn main() -> Result<(), Box<dyn Error>> {
    let p = PuzzleInput::init(None)?;

    println!("\n---\nReading file {:?} into a vector and printing it.", p.file_input);
    p.vectorized()?
        .iter()
        .for_each(|x| println!("{:?}", x));


    println!("\n---\nReading file {:?} as a buffered input and printing each line.", p.file_input);
    p.bufferized()?
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .for_each(|y| println!("{:?}", y));

    Ok(())
}
