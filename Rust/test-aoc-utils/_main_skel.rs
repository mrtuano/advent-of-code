
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn puzzle_solve1<T>(data: &Vec<String>) -> Result<T, String> {
    todo!();
    Ok(results)
}

fn puzzle_solve2<T>(data: &Vec<String>) -> Result<T, String> {
    todo!();
    Ok(results)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day XX <<<<<<<<<<\n");

    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------");
    println!("  Part 1 Result: {:?}\n", puzzle_solve1(&data)?);

    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------");
    println!("  Part 2 Result: {:?}\n", puzzle_solve2(&data)?);

    Ok(())
}



/* *************************************************************************
                            TESTING
   ************************************************************************* */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_solve1() -> Result<(), String> {
        
        // Update as needed
        let test_input = "test.data";
        let test_expected = 0u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve1(&d)?, Ok(test_expected));

        Ok(())
    }
   
    #[test]
    fn test_puzzle_solve2() -> Result<(), String> {

        // Update as needed
        let test_input = "test.data";
        let test_expected = 0u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, Ok(test_expected));

        Ok(())
    }
}