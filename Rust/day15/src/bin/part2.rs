// ************************************************************************* //
//                          M A I N    P R O G R A M
// ************************************************************************* //

use day15::part2::solve_part2;
use aoc_utils::aoc_utils::*;

fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 15 <<<<<<<<<<\n");

    println!("---------------------");
    println!("Calculating Part 2...");
    println!("---------------------\n");
    println!("\n    >>> Part 2 Result: {:?} <<<\n\n", solve_part2(&data)?);

    Ok(())
}