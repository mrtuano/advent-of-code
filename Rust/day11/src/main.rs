
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

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

fn apply_rules(items: Vec<usize>) -> Result<Vec<usize>, String> {
    let mut result_numbers: Vec<usize> = vec![];

    for number in items.iter() {

        // Rule 1
        if *number == 0 {
            result_numbers.push(1usize);
            continue;
        }

        // Rule 2
        if let Some(mut rule_result) = split_even_number(number) {
            result_numbers.append(&mut rule_result);
            continue;
        }
        
        // Rule 3
        result_numbers.push(*number * 2024);
    }

    Ok(result_numbers)
}

fn puzzle_solve1(data: &Vec<String>, blinks: u32) -> Result<usize, String> {

    let init_items: Vec<usize> = data.concat().split_whitespace()
        .filter_map(|x|safe_parse(x).ok())
        .collect();

    let mut result_items: Vec<usize> = init_items;

    // TODO: Remove, For debugging only
    //println!("numbers: {:?}\n", result_items);

    for _ in 0..blinks {
        result_items = apply_rules(result_items)?;
        // TODO: Remove, For debugging only
        //println!("numbers: {:?}\n", result_items);

    }
 
    Ok(result_items.iter().count())
}

fn puzzle_solve2(data: &Vec<String>, blinks: u32) -> Result<usize, String> {
    puzzle_solve1(data, blinks)
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
///*
    // --------------- PART 1 --------------- //
    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------\n");

    let blinks = 25u32;

    println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&data, blinks)?);
//*/
/*
    // --------------- PART 2 --------------- //
    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------\n");

    let blinks = 75u32;

    println!("  Part 2 Result: {:?}\n\n", puzzle_solve2(&data, blinks)?);
*/
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
        let test_expected = 55312usize;

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
        let test_expected = 0usize;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d, test_input2)?, test_expected);

        Ok(())
    }
}