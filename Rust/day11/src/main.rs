
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::HashMap;

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            CUSTOM TYPES
   ************************************************************************* */


/* *************************************************************************
                            CUSTOM TRAITS
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
fn split_even_number(number: &usize) -> Option<Vec<usize>> {
    let number_str = number.to_string();
    let lenght = number_str.len();
    if lenght % 2 == 0 {
        let half = lenght / 2;
        let first = &number_str[..half].parse().ok()?;
        let second = &number_str[half..].parse().ok()?;
        Some(vec![*first, *second])
    } else {
        None
    }
}

// ----------------------------------------------------
// Solution based on Chris Biscardi video
//   --> https://www.youtube.com/watch?v=-DlwYW6TxIU
// ----------------------------------------------------
fn solve_puzzle(data: &Vec<String>, blinks: u32) -> Result<u64, String> {

    let init_items: Vec<u64> = data.concat().split_whitespace()
        .filter_map(|x|safe_parse64(x).ok())
        .collect();

    let mut num_counts: HashMap<u64, u64> = HashMap::default();

    for i in init_items.iter() {
        num_counts.entry(*i)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    for _ in 0..blinks {
        let mut nums_cache: HashMap<u64, u64> = HashMap::default();
        for (num, count) in num_counts.into_iter() {
            match num {
                0 => {
                    nums_cache.entry(1)
                        .and_modify(|x|*x += count)
                        .or_insert(count);
                },
                a if (a.checked_ilog(10).unwrap_or(0) + 1) % 2 == 0 => {
                    if let Some(evens ) = split_even_number(&(a as usize)) {
                        nums_cache.entry(evens[0] as u64)
                            .and_modify(|x| *x += count)
                            .or_insert(count);
                        nums_cache.entry(evens[1] as u64)
                            .and_modify(|x| *x += count)
                            .or_insert(count);
                    }
                },
                b => {
                    nums_cache.entry(b * 2024)
                        .and_modify(|x| *x += count)
                        .or_insert(count);

                }
            };
        }
        num_counts = nums_cache;
    }

    Ok(num_counts.values().sum::<u64>())
}

fn puzzle_solve1(data: &Vec<String>, blinks: u32) -> Result<u64, String> {

    if let Ok(x)= solve_puzzle(data, blinks) {
        Ok(x)
    } else {
        Err(format!("Cannot parse numbers!"))
    }

}

fn puzzle_solve2(data: &Vec<String>, blinks: u32) -> Result<u64, String> {

    if let Ok(x)= solve_puzzle(data, blinks) {
        Ok(x)
    } else {
        Err(format!("Cannot parse numbers!"))
    }
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 11 <<<<<<<<<<\n");

    // --------------- PART 1 --------------- //
    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------\n");

    let blinks = 25u32;

    println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&data, blinks)?);

    // --------------- PART 2 --------------- //
    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------\n");

    let blinks = 75u32;

    println!("  Part 2 Result: {:?}\n\n", puzzle_solve2(&data, blinks)?);

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
        let test_input = "test.data";    // list of numbers
        let test_input2 = 25u32;          // number of blinks
        let test_expected = 55312u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve1(&d, test_input2)?, test_expected);

        Ok(())
    }
   
    #[test]
    fn test_puzzle_solve2() -> Result<(), String> {

        // Update as needed
        let test_input = "test.data";
        let test_input2 = 75u32;          // number of blinks
        let test_expected = 0u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d, test_input2)?, test_expected);

        Ok(())
    }
}