
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::{collections::HashMap, u64};

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                        CONSTANTS AND CUSTOM TYPES
   ************************************************************************* */
const MAX_PRESS: u64 = 100;

type Point = (u64, u64);


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
    tokens: u64,
    x: u64,
    y: u64
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

    /*
        Two variable linear equation in disguise:
            (number-of-buttonA-press * buttonA.X) + (number-of-buttonB-press * buttonB.X) = prize.X
            (number-of-buttonA-press * buttonA.Y) + (number-of-buttonB-press * buttonB.Y) = prize.Y
     */
    fn play_machine(&self) -> Option<(u64, u64)> {

        // --------------------------------
        // Solve for button A presses
        // Equation: 
        //     let press_a = ((self.prize.0 * self.b.y) - (self.b.x * self.prize.1)) / 
        //                   ((self.a.x * self.b.y) - (self.b.x * self.a.y));
        // --------------------------------
        let test_num = (self.prize.0 * self.b.y).checked_sub(self.b.x * self.prize.1);
        let test_den = (self.a.x * self.b.y).checked_sub(self.b.x * self.a.y);

        let (num, den) = match (test_num, test_den) {
            (Some(n), Some(d)) => (n, d),
            (None, None) => {
                let test_num2 = (self.b.x * self.prize.1).checked_sub(self.prize.0 * self.b.y);
                let test_den2 = (self.b.x * self.a.y).checked_sub(self.a.x * self.b.y);
                let (n2, d2) = match (test_num2, test_den2) {
                    (Some(a), Some(b)) => (a, b),
                    (_, _) => return None
                };
                (n2, d2)
            },
            (_, _) => return None
        };
        
        let press_a = num.checked_div(den);


        // --------------------------------
        // Solve for button B presses
        // Equation: 
        //     let press_b = ((self.prize.1 * self.a.x) - (self.prize.0 * self.a.y)) / 
        //                   ((self.a.x * self.b.y) - (self.b.x * self.a.y));
        // --------------------------------

        let test_num = (self.prize.1 * self.a.x).checked_sub(self.prize.0 * self.a.y);
        let test_den = (self.a.x * self.b.y).checked_sub(self.b.x * self.a.y);

        let (num, den) = match (test_num, test_den) {
            (Some(n), Some(d)) => (n, d),
            (None, None) => {
                let test_num2 = (self.prize.0 * self.a.y).checked_sub(self.prize.1 * self.a.x);
                let test_den2 = (self.b.x * self.a.y).checked_sub(self.a.x * self.b.y);
                let (n2, d2) = match (test_num2, test_den2) {
                    (Some(a), Some(b)) => (a, b),
                    (_, _) => return None
                };
                (n2, d2)
            },
            (_, _) => return None
        };
        
        let press_b = num.checked_div(den);

        // --------------------------------
        // We can have 0 press on either one, but not both
        // If both 0 presses, then invalid machine
        // --------------------------------
        match (press_a, press_b) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(a), None) => Some((a, 0)),
            (None, Some(b)) => Some((0, b)),
            (None, None) => None
        }
    }
}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

// AI generated code
fn parse_location_data(data: Option<&&str>, prize_factor: u64) -> Option<Point> {
    if let Some(d) = data {
        let parts: Vec<&str> = d.split(&['=', ','][..]).collect();
        let x = parts[1].trim().parse().unwrap_or(0);
        let y = parts[3].trim().parse().unwrap_or(0);
        let p: Point = (x + prize_factor, y + prize_factor);
        Some(p)
    } else {
        None
    }
}

fn parse_button_data(data: Option<&&str>) -> Option<(u64, u64)> {
    if let Some(d) = data {
        let parts: Vec<&str> = d.split(|c| c == ' ' || c == ',' || c == '+').collect();
        //dbg!(format!("parse_button_data input: {:?}", parts));
        let x = parts.get(3)?.to_string().parse::<u64>().ok().unwrap_or(0);
        let y = parts.get(6)?.to_string().parse::<u64>().ok().unwrap_or(0);
        //dbg!(format!("parse button data: {:?} {:?}", x, y));
        Some((x, y))
    } else {
        None
    }
}

fn make_machine(data: &HashMap<&str, &str>, prize_factor: u64) -> Option<Machine> {
    let a = Button::button_a(data.get(&"a"));
    let b = Button::button_b(data.get(&"b"));
    let prize= parse_location_data(data.get(&"p"), prize_factor);
    if a.is_some() && b.is_some() && prize.is_some() {
        Some(Machine::init(a?, b?, prize?))
    } else {
        None
    }
}

fn read_input(data: &Vec<String>, prize_factor: u64) -> Vec<Machine> {
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
            if let Some(machine) = make_machine(&buffer, prize_factor) {
                machines.push(machine);
            }
            buffer.clear();
        }
    }
    machines
}

fn solve_puzzle(data: &Vec<String>, p: u64, max_presses: u64) -> Result<u64, String> {

    let mut tokens = 0u64;

    let machines = read_input(data, p);

    // TODO: Remove for debugging only
    //dbg!(&machines);

    for m in machines.iter() {

        // TODO: Remove for debugging only
        //println!("\n----------------------");
        //println!("Machine:");
        //println!("\tButton A: {:?}X {:?}Y", m.a.x, m.a.y);
        //println!("\tButton B: {:?}X {:?}Y", m.b.x, m.b.y);
        //println!("\tPrize: {:?}", m.prize);

        if let Some((press_a, press_b)) = m.play_machine() {

            if press_a <= max_presses && press_b <= max_presses {

                if press_a * m.a.x + press_b * m.b.x == m.prize.0 &&
                    press_a * m.a.y + press_b * m.b.y == m.prize.1 {

                    let machine_tokens = (m.a.tokens * press_a + m.b.tokens * press_b) as u64;
                    tokens += machine_tokens;

                    // TODO: Remove for debugging only
                    //println!("\tPress A: {:?} | Press B: {:?} | Tokens: {:?}", press_a, press_b, machine_tokens);
                }
            } else {
                //println!("\tPress A: {:?} | Press B: {:?} | TOO MANY BUTTON PRESSES!", press_a, press_b);
                ()
            }

        } else {
            //println!("\t\tINVALID MACHINE!")
            ()
        }
    }

    Ok(tokens)
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {
    solve_puzzle(data, 0, MAX_PRESS)
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u64, String> {
    solve_puzzle(data, 10000000000000, u64::MAX)
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