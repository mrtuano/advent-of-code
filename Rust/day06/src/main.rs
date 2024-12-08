
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Corner {
    position: Position,
    direction: Direction
}

#[derive(Clone, Debug)]
struct PatrolLoop {
    corners: Vec<Corner>,
    last_corner: Option<Corner>
}

impl PatrolLoop {
    fn new(corner: Option<(Position, Direction)>) -> PatrolLoop {
        let mut ploop = PatrolLoop {
            corners: vec![],
            last_corner: None
        };
        match corner {
            Some(p) => {
                let c = Corner { position: p.0, direction: p.1 };
                ploop.corners.push(c);
            },
            None => ()
        };
        ploop
    }

    fn add_corner(&mut self, corner: (Position, Direction)) {
        let c = Corner { position: corner.0, direction: corner.1 };
        self.corners.push(c);
    }

    fn check_loop(&mut self, guard: Guard, grid: &Grid) -> bool {
        if self.corners.len() == 3 {
            if self.last_corner.is_none() {
                if let last_corner = get_last_corner(&self.corners) {
                    self.last_corner = last_corner;
                }
            }
            if completes_loop(self, &guard, grid) {
                self.corners.clear();
                return true;
            }
            if past_last_corner(self, &guard) {
                self.corners.clear();
                return false;
            }
            return false;
        } else if self.corners.len() > 3 {
            self.corners.clear();
            return false;
        } else {
            return false;
        }
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
        self.direction = new_direction;
        self.vector = vector(&new_direction);
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

    fn possible_infinite_loops(&mut self, grid: &Grid) -> HashSet<Position> {

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
fn past_last_corner(patrol_loop: &PatrolLoop, guard: &Guard) -> bool {
    todo!();
    false
}

fn completes_loop(patrol_loop: &PatrolLoop, guard: &Guard, grid: &Grid) -> bool {
    if let Some(last_corner) = &patrol_loop.last_corner {
        if last_corner.position == guard.position && 
            last_corner.direction == swivel_left(&guard.direction) &&
            whats_ahead(&guard.position, &guard.direction, grid) == Facing::Nothing {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn swivel_right(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn swivel_left(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn get_last_corner(corners: &Vec<Corner>) -> Option<Corner> {

    let mut vertical_distance = 0i32;
    let mut horizontal_distance = 0i32;

    if corners.is_empty() || corners.len() < 3 {
        return None;
    } 

    let first = &corners[0];
    let second = &corners[1];
    let third = &corners[2];

    if first.direction == Direction::North || first.direction == Direction::South {
        vertical_distance = (first.position.y - second.position.y).abs();
        horizontal_distance = (third.position.x - second.position.x).abs();
    } else if first.direction == Direction::East || first.direction == Direction::West {
        horizontal_distance = (second.position.x - first.position.x).abs();
        vertical_distance = (third.position.y - second.position.y).abs();
    }

    let fourth_possible_dir = swivel_left(&first.direction);

    let mut fourth_possible_pos = first.position.clone();
    match fourth_possible_dir {
        Direction::North => fourth_possible_pos.y = first.position.y + vertical_distance,
        Direction::East => fourth_possible_pos.x = first.position.x - horizontal_distance,
        Direction::South => fourth_possible_pos.y = first.position.y - vertical_distance,
        Direction::West => fourth_possible_pos.x = first.position.x + horizontal_distance
    };

    Some(Corner {
        position: fourth_possible_pos,
        direction: fourth_possible_dir
    })
}

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

    let grid: Grid = Grid::init(data);
    //println!("grid: {:?}", &grid);

    let mut guard = Guard::init(grid.starting, Direction::North);

    let possible_obstructions: HashSet<Position> = guard.possible_infinite_loops(&grid);

    let results = possible_obstructions.len() as u32;

    Ok(results)
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
        let test_expected = 6u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}