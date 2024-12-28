
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::collections::HashMap;

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                        CONSTANTS AND CUSTOM TYPES
   ************************************************************************* */
const MAX_PRESS: u32 = 100;

type Point = (u32, u32);


/* *************************************************************************
                            CUSTOM TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

// -------------------- Claw Machine Buttons -------------------- //
#[derive(Clone, Copy, Debug)]
struct Button {
    tokens: u8,
    x: u32,
    y: u32
}

impl Button {
    fn button_a(data: Option<&&str>) -> Option<Button> {
        if let Some((x, y)) = parse_button_data(data) {
            Some(Button { tokens: 3, x, y})
        } else {
            None
        }
    }

    fn button_b(data: Option<&&str>) -> Option<Button> {
        if let Some((x, y)) = parse_button_data(data) {
            Some(Button { tokens: 1, x, y})
        } else {
            None
        }
    }

    fn press(&self, location: &Point) -> Point {
        (location.0 + self.x, location.1 + self.y)
    }
}

// ------------------------ Claw Machine ------------------------ //
#[derive(Clone, Copy, Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize: Point
}

impl Machine {
    fn init(a: Button, b: Button, prize: Point) -> Self {
        Self { a, b, prize }
    }
}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

// AI generated code
fn parse_location_data(data: Option<&&str>) -> Option<Point> {
    if let Some(d) = data {
        let parts: Vec<&str> = d.split(&['=', ','][..]).collect();
        let x = parts[1].trim().parse().unwrap_or(0);
        let y = parts[3].trim().parse().unwrap_or(0);
        let p: Point = (x, y);
        Some(p)
    } else {
        None
    }
}

// AI generated code
fn parse_button_data(data: Option<&&str>) -> Option<(u32, u32)> {
    if let Some(d) = data {
        let parts: Vec<&str> = d.split(|c| c == ' ' || c == ',' || c == '+').collect();
        //dbg!(format!("parse_button_data input: {:?}", parts));
        let x = parts.get(3)?.to_string().parse::<u32>().ok().unwrap_or(0);
        let y = parts.get(6)?.to_string().parse::<u32>().ok().unwrap_or(0);
        //dbg!(format!("parse button data: {:?} {:?}", x, y));
        Some((x, y))
    } else {
        None
    }
}

fn make_machine(data: &HashMap<&str, &str>) -> Option<Machine> {
    let a = Button::button_a(data.get(&"a"));
    let b = Button::button_b(data.get(&"b"));
    let prize= parse_location_data(data.get(&"p"));
    if a.is_some() && b.is_some() && prize.is_some() {
        Some(Machine::init(a?, b?, prize?))
    } else {
        None
    }
}

fn read_input(data: &Vec<String>) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];
    let mut buffer: HashMap<&str, &str> = HashMap::new();
    for line in data.iter() {
        if line.contains("Button A") {
            buffer.entry(&"a").or_insert(line);
        } else if line.contains("Button B") {
            buffer.entry(&"b").or_insert(line);
        } else if line.contains("Prize:") {
            buffer.entry(&"p").or_insert(line);
        }
        if buffer.contains_key(&"a") && buffer.contains_key(&"b") 
            && buffer.contains_key(&"p") {
            if let Some(machine) = make_machine(&buffer) {
                machines.push(machine);
            }
            buffer.clear();
        }
    }
    machines
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {
    let mut tokens = 0u64;
    let machines = read_input(data);

    // TODO: Remove for debugging only
    //dbg!(&machines);

    Ok(tokens)
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u64, String> {
    let mut tokens = 0u64;
    Ok(tokens)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 13 <<<<<<<<<<\n");

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
        let test_expected = 480u64;

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