
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::HashMap;

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            CUSTOM TYPES
   ************************************************************************* */
type Point = (i32, i32);


/* *************************************************************************
                            CUSTOM TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */
enum Quadrant {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4
}


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
#[derive(Clone, Copy, Debug)]
struct Robot {
    initial: Point,
    current: Point,
    vx: i32,
    vy: i32
}

impl Robot {

    fn init(input: &String) -> Option<Robot> {

        let parts: Vec<&str> = input.split_whitespace().collect();

        let p_values: Option<Vec<i32>> = parts.get(0)
            .and_then(|s| s.get(2..))
            .map(|s| s.split(',').map(|s| s.parse().ok()).collect())
            .and_then(|vec: Vec<Option<i32>>| 
                if vec.iter().all(Option::is_some) { 
                    Some(vec.into_iter().map(Option::unwrap).collect()) 
                } else { 
                    None 
                }
            );

        let v_values: Option<Vec<i32>> = parts.get(1)
            .and_then(|s| s.get(2..))
            .map(|s| s.split(',').map(|s| s.parse().ok()).collect())
            .and_then(|vec: Vec<Option<i32>>| 
                if vec.iter().all(Option::is_some) { 
                    Some(vec.into_iter().map(Option::unwrap).collect()) 
                } else { 
                    None 
                }
            );

        match (p_values, v_values) {
            (Some(p), Some(v)) if p.len() == 2 && v.len() == 2 => Some(Robot {
                initial: (p[0], p[1]),
                current: (p[0], p[1]),
                vx: v[0],
                vy: v[1],
            }),
            _ => None,
        }
    }

    fn position_after_delta(&mut self, max: Point, delta_time: u32) -> Option<Point> {
        for i in 1..=delta_time {
            self.move_position(max);
            // TODO, Remove for debugging only
            //println!("Pos {:?}: {:?}", i, self.current);
        }
        Some((self.current.0, self.current.1))
    }

    fn move_position(&mut self, max: Point) {
        self.current.0 =  (self.current.0 + self.vx).rem_euclid(max.0);
        self.current.1 = (self.current.1 + self.vy).rem_euclid(max.1);

        /*let new_x = self.current.0 + self.vx;
        if new_x > max.0 {
            self.current.0 = new_x - max.0 - 1;
        } else if new_x < 0 {
            self.current.0 = new_x + max.0;
        } else {
            self.current.0 = new_x;
        }

        let new_y = self.current.1 + self.vy;
        if new_y > max.1 {
           self.current.1 = new_y - max.1; 
        } else if new_y < 0 {
           self.current.1 = new_y + max.1 + 1; 
        } else {
            self.current.1 = new_y;
        }*/
    }
}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn which_quadrant(position: &Point, mid: &Point, max: &Point) -> Option<Quadrant> {

    // if robot is in vertical or horizontal mid line, skip
    if position.0 == mid.0 || position.1 == mid.1 {
        // TODO, Remove for debugging only
        //println!("r pos: {:?}", position);
        return None;
    }

    // Quadrant initialize
    let q1_limits: HashMap<String, Point> = HashMap::from([("min".to_string(), (0, 0))        , ("max".to_string(), (mid.0, mid.1))]);
    let q2_limits: HashMap<String, Point> = HashMap::from([("min".to_string(), (mid.0, 0))    , ("max".to_string(), (max.0, mid.1))]);
    let q3_limits: HashMap<String, Point> = HashMap::from([("min".to_string(), (0, mid.1))    , ("max".to_string(), (mid.0, max.1))]);
    let q4_limits: HashMap<String, Point> = HashMap::from([("min".to_string(), (mid.0, mid.1)), ("max".to_string(), (max.0, max.1))]);

    if position.0 >= q1_limits["min"].0 && position.0 <= q1_limits["max"].0 && position.1 >= q1_limits["min"].1 && position.1 <= q1_limits["max"].1 {
        Some(Quadrant::One)
    } else if position.0 >= q2_limits["min"].0 && position.0 <= q2_limits["max"].0 && position.1 >= q2_limits["min"].1 && position.1 <= q2_limits["max"].1 {
        Some(Quadrant::Two)
    } else if position.0 >= q3_limits["min"].0 && position.0 <= q3_limits["max"].0 && position.1 >= q3_limits["min"].1 && position.1 <= q3_limits["max"].1 {
        Some(Quadrant::Three)
    } else if position.0 >= q4_limits["min"].0 && position.0 <= q4_limits["max"].0 && position.1 >= q4_limits["min"].1 && position.1 <= q4_limits["max"].1 {
        Some(Quadrant::Four)
    } else {
        None
    }
}

fn read_data(data: &Vec<String>) -> Result<Vec<Robot>, String> {
    let mut robots: Vec<Robot> = vec![];

    for l in data.iter() {
        if let Some(r) = Robot::init(l) {
            // TODO: Remove for debugging only
            println!("robot before: {:?}", r.current);
            robots.push(r);
        }
    }
    Ok(robots)
}

/* ============================================================================
    Process Puzzle Part 1:

    1. Read the input --> Robot initial position and velocity
    2. Move the robots after 100 seconds
    3. Get the position of robots after 100 seconds
    4. Sort each robot to quadrants based on position 

                               |
              Quadrant 1       |        Quadrant 2
                               |
            -------------------------------------
                               |
              Quadrant 3       |        Quadrant 4
                               |

    5. Count each number of each robots per quadrant
    6. Multiply the counts to get the result
   ========================================================================== */
fn puzzle_solve1(data: &Vec<String>, input_wide: i32, input_tall: i32, delta_time: u32) -> Result<u64, String> {

    // Get the robots from input
    let mut robots = read_data(data)?;

    // Max point and mid-points
    let max: Point = (input_wide - 1, input_tall - 1);
    let mid: Point = (max.0/2, max.1/2);

    // TODO: Remove for debugging only
    println!("Max: {:?}, Mid: {:?}", &max, &mid);

    let mut q1: Vec<&Robot> = vec![];
    let mut q2: Vec<&Robot> = vec![];
    let mut q3: Vec<&Robot> = vec![];
    let mut q4: Vec<&Robot> = vec![];

    // Move the robots after some time
    for r in robots.iter_mut() {

        // Move robot
        r.position_after_delta(max, delta_time);

        // TODO: Remove for debugging only
        println!("robot after: {:?}", r.current);

        // Classify robot if in a quadrant or if on mid-line (horiziontal or vertical)
        match which_quadrant(&r.current, &mid, &max) {
            Some(Quadrant::One) => q1.push(r),
            Some(Quadrant::Two) => q2.push(r),
            Some(Quadrant::Three) => q3.push(r),
            Some(Quadrant::Four) => q4.push(r),
            None => continue
        };
    }

    // TODO: Remove, for debugging only
    println!("q1: {:?}", q1.len());
    println!("q2: {:?}", q2.len());
    println!("q3: {:?}", q3.len());
    println!("q4: {:?}", q4.len());

    let safety_factor = q1.len() * q2.len() * q3.len() * q4.len();

    Ok(safety_factor as u64)

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

    println!("\n>>>>>>>>>>> Puzzle Day 14 <<<<<<<<<<\n");

    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------\n");
    println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&data, 101i32, 103i32, 100u32)?);

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
        let test_expected = 12u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve1(&d, 11i32, 7i32, 100u32)?, test_expected);

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