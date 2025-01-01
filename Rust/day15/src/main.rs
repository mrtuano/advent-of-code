
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            CUSTOM TYPES
   ************************************************************************* */
type Point = (u32, u32);

/* *************************************************************************
                            CUSTOM TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */
#[derive(Debug)]
enum Direction {
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Wall,
    Box,
    Robot
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

// --------------------- Robot -------------------- //
// Special type of Object that will manage the
// positions of the other objects in the warehouse
// i.e. "ObjectsManager"
// ------------------------------------------------ //
#[derive(Debug)]
struct Robot {
    initial: Point,
    current: Point,
}

impl Robot {
    fn init(p: Point) -> Self {
        Self { initial: p, current: p}
    }

    fn set_position(&mut self, p: Point) {
        self.current = p;
    }
}

// ------------- Objects in Warehouse ------------- //
// Types:
//    # - Wall, immovable object
//    O - Box, movable object
//    @ - Robot, object that initiates movement
//    . - Empty space, (Rust Option::None enum)
// ----------------------------------------------- //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Objects {
    id: u32,
    kind: Option<Object>,
    position: Point
}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

fn read_data(data: &Vec<String>) -> Option<Objects> {

}

fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {
    todo!();
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u64, String> {
    todo!();
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
    println!("---------------\n");
    println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&data)?);

    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------\n");
    println!("  Part 2 Result: {:?}\n\n", puzzle_solve2(&data)?);

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
        let test_expected = 10092u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve1(&d)?, test_expected);

        Ok(())
    }
   
    #[test]
    fn test_puzzle_solve2() -> Result<(), String> {

        // Update as needed
        let test_input = "test.data";
        let test_expected = 0u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}