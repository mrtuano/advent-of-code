
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::cmp::Ordering;
use std::collections::HashSet;

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            CUSTOM TYPES
   ************************************************************************* */


/* *************************************************************************
                            TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug)]
enum Facing {
    Limbo,
    Nothing,
    Obstacle
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn init(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    fn peek_ahead(&self, v: (i8, i8)) -> Position {
        Position {
            x: self.x + v.0 as i32,
            y: self.y + v.1 as i32
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .cmp(&other.x)
            .then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Grid {
    min: Position,
    max: Position,
    obstacles: HashSet<Position>,
    starting: Position // The guard's starting position
}

impl Grid {
    fn init(input: &Vec<String>) -> Self {

        let max_y: i32 = input.len() as i32;
        let max_x: &i32 = &input[0].chars().fold(0,|acc, _| acc + 1);

        let mut obstacles: HashSet<Position> = HashSet::new();
        let mut starting: Position = Position::init(0, 0);

        // We assume the following:
        // - Only '#' are present in the grid representing obstacles
        // - The guard, '^', starting direction is facing north    
        for (i, line) in input.iter().enumerate() {
            let cols: Vec<char> = line.chars().collect();
            let pos_y = i as i32;
            //println!("{:?} {:?}", pos_y, line);
            for (j, c) in cols.iter().enumerate() {
                let pos_x = j as i32;
                match c {
                    '#' => { obstacles.insert(Position::init(pos_x, pos_y)); true },
                    '^' => { starting = Position::init(pos_x, pos_y); true },
                    _ => continue
                };
            }

        }

        Self { 
            min: Position::init(0,0),
            max: Position::init(*max_x-1, max_y-1),
            obstacles,
            starting
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
    vector: (i8, i8)
}

impl Guard {
    fn init(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            vector: vector(&direction)
        }
    }

    fn turn_right(&mut self) {
        let new_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };
        let new_vector = vector(&new_direction);
        self.direction = new_direction;
        self.vector = new_vector;
    }

    fn move_forward(&mut self) {
        let new_position = self.position.peek_ahead(self.vector);
        self.position = new_position
    }

    fn facing_obstacle(&self, grid: &Grid) -> bool {
        //println!("Facing obstacle?");
        match whats_ahead(&self.position, &self.direction, grid) {
            Facing::Obstacle => true,
            _ => false
        }
    }

    fn exiting_edge(&self, grid: &Grid) -> bool {
        //println!("Exiting edge?");
        match whats_ahead(&self.position, &self.direction, grid) {
            Facing::Limbo => true,
            _ => false
        }
    }

    fn start_patrolling(&mut self, grid: &Grid) -> HashSet<Position> {

        let mut patrol_path: HashSet<Position> = HashSet::new();
        patrol_path.insert(self.position.clone());

        while !self.exiting_edge(grid) {
            if self.facing_obstacle(grid) {
                self.turn_right();
            } else {
                self.move_forward();
            }
            //println!("guard is at position: {:?}", self.position);
            patrol_path.insert(self.position.clone());
        }

        patrol_path
    }

}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn whats_ahead(pos: &Position, dir: &Direction, grid: &Grid) -> Facing {
    let v = vector(dir);
    let peeked = pos.peek_ahead(v);
    //println!("peeked: {:?}", &peeked);
    if peeked.x < grid.min.x || peeked.y < grid.min.y || peeked.x > grid.max.x || peeked.y > grid.max.y {
        //println!("facing limbo");
        Facing::Limbo
    } else if grid.obstacles.contains(&peeked) {
        //println!("facing obstacle");
        Facing::Obstacle
    } else {
        //println!("clear ahead");
        Facing::Nothing
    }
}

fn vector(direction: &Direction) -> (i8, i8) {
    match *direction {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0)
    }
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u32, String> {

    let grid: Grid = Grid::init(data);
    //println!("grid: {:?}", &grid);

    let mut guard = Guard::init(grid.starting, Direction::North);

    let patrol_path: HashSet<Position> = guard.start_patrolling(&grid);
    let results = patrol_path.len() as u32;

    Ok(results)
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u32, String> {
    todo!();
    //let mut results = 0u32;
    //Ok(results)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    //let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 06 <<<<<<<<<<\n");

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
        let test_expected = 41u32;

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
        let test_expected = 0u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}