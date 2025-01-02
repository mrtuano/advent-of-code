
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */
//use std::collections::{HashMap, HashSet};
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
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn vector(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0)
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Wall,
    Box,
    Robot,
    None
}

impl Type {
    fn draw(&self) -> char {
        match self {
            Type::Wall => '#',
            Type::Box => 'O',
            Type::Robot => '@',
            Type::None => '.'
        }
    }

    fn parse(c: &char) -> Type {
        match c {
            '#' => Type::Wall,
            '@' => Type::Robot,
            'O' => Type::Box,
            _ => Type::None
        }
    }
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

// ----------------------------------------------- //
// The Objects in the Warehouse
// Separately track the position of the robot.
// ----------------------------------------------- //
#[derive(Debug)]
struct Warehouse {
    objects: HashMap<Point, Type>,
    robot: Point
}

impl Warehouse {
    fn robot_attempt_move(&self, direction: &Direction) -> Option<Vec<(Point, Point)>> {

        let mut buffer: Vec<(Point, Point)> = vec![];
        let mut current: Point = self.robot;
        let v = direction.vector();

        let max: &Point = self.objects.keys().max()?;

        // Limit our loop so that we won't go on forever and ever
        for _i in 0..max.0 {
            if let Some(ahead) = peek_ahead(&current, v) {
                // TODO: Remove, for debugging only
                //print!("Ahead {:?}, ", &ahead);
                if let Some(o) = self.objects.get(&ahead) {
                    // TODO: Remove, for debugging only
                    //println!("{:?}", &o);
                    if *o == Type::Box {
                        buffer.push((current, ahead));
                        current = ahead;
                    } else if *o == Type::None {
                        buffer.push((current, ahead));
                        return Some(buffer);
                    } else if *o == Type::Wall {
                        return None;
                    }
                }
            };
        }

        None
    }

    fn dump(&self) {
        let max: &Point = self.objects.keys().max().unwrap_or(&(0,0));
        for y in 0..=max.1 {
            let mut line_chars: Vec<char> = vec![];
            for x in 0..=max.0 {
                if let Some(o) = self.objects.get(&(x, y)) {
                    let s = o.draw();
                    line_chars.push(s);
                }
            }
            let line: String = line_chars.iter().collect();
            println!("{:?}", line);
        }
    }
}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn peek_ahead(p: &Point, v: (i8, i8)) -> Option<Point> {
    Some(
        (
            p.clone().0.checked_add(v.0 as i32)?,
            p.clone().1.checked_add(v.1 as i32)?
        )
    )
}

fn read_data(data: &Vec<String>) -> (Warehouse, Vec<Direction>) {

    let mut robot: Point = (0, 0);
    let mut objects: HashMap<Point, Type> = HashMap::new();
    let mut directions: Vec<Direction> = vec![];

    for (y, l) in data.iter().enumerate() {
        // Parse the warehouse map
        if l.contains('#') {
            for (x, c) in l.chars().enumerate() {
                let p: Point = (x as i32, y as i32);
                let t = Type::parse(&c);
                objects.entry(p).or_insert(t);
                if t == Type::Robot {
                    robot = p;
                }
            }
        } else if l.contains(['<', 'v', '>', '^']) {
        // Parse the robot directions
            for c in l.chars() {
                let d = match c {
                    '<' => Direction::Left,
                    'v' => Direction::Down,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    _ => continue
                };
                directions.push(d);
            }

        }
    }

    (Warehouse {objects, robot}, directions)
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {

    let (mut warehouse, directions) = read_data(data);

    //dbg!(&warehouse);
    //dbg!(&directions);

    // TODO: Remove, for debugging only.
    println!("\nBefore:");
    warehouse.dump();

    for d in directions.iter() {

        // TODO: Remove, for debugging only.
        //println!("\nDirection: {:?}", d);

        if let Some(new_positions) = warehouse.robot_attempt_move(d) {
            for (old_position, new_position) in new_positions.iter().rev() {
                // TODO: Remove, for debugging only.
                //println!("old: {:?} New: {:?}", old_position, new_position);
                if let Some(o) = warehouse.objects.remove(old_position) {
                    if o == Type::Robot {
                        warehouse.objects.insert(*old_position, Type::None);
                        warehouse.robot = *new_position;
                    }
                    warehouse.objects.insert(*new_position, o);
                }
            }
        }

        // TODO: Remove, for debugging only.
        //warehouse.dump();
    }
    
    // TODO: Remove, for debugging only.
    println!("\nAfter:");
    warehouse.dump();

    let sum_gps: u64 = warehouse.objects.iter()
        //.filter(|(p, o)| o.kind == Type::Box)
        .filter(|(_p, o)| **o == Type::Box)
        .map(|(p, _)| p.0 as u64 + 100*p.1 as u64)
        .sum();

    Ok(sum_gps)
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