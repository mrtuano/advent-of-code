
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec;

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

// The weights signify we can only turn right based on our scenario
// Hence when we 'turn_right' (or 'rotate-left') it goes to the next value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North = 1,
    East = 2,
    South = 3,
    West = 4
}

impl Direction {
    fn circular() -> VecDeque<Direction> {
        let mut v: VecDeque<Direction> = VecDeque::new();
        v.push_back(Direction::North);
        v.push_back(Direction::East);
        v.push_back(Direction::South);
        v.push_back(Direction::West);
        v
    }
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
#[derive(Clone, Copy, Debug, Eq, Hash)]
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

    fn same(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
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

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Corner {
    position: Position,
    direction: Direction
}

impl PartialEq for Corner {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && 
            self.direction == other.direction
    }
}

// We are fortunate our loops are rectangular...
// ...there are four corners in our loop. :)
#[derive(Clone, Debug)]
struct PatrolLoop {
    corners_count: HashMap<Corner, u32>,
    four_corners: VecDeque<Corner>
}

impl PatrolLoop {
    fn new() -> Self {
        Self {
            corners_count: HashMap::new(),
            four_corners: VecDeque::new()
        }
    }

    fn reset(&mut self) {
        self.corners_count.clear();
        self.four_corners.clear();
    }

    fn add_corner(&mut self, corner: (Position, Direction)) {
        let c = Corner { position: corner.0, direction: corner.1 };
        *self.corners_count.entry(c).or_insert(0) += 1;

        // in a loop, the last 4 corners of a rectangular loop 
        // are repeated infinitely, hence we only track the last 4 
        // corners everytime we make a turn in a double-ended queue
        if self.four_corners.len() < 4 {
            self.four_corners.push_back(c);
        } else {
            self.four_corners.pop_front();
            self.four_corners.push_back(c);
        }
    }

    fn check_loop(&mut self) -> bool {

        /*
            If we are in a rectangular loop, there are four corners
            and if there are more, it means we made more than 4 turns in our grid,
            which doesn't matter in our scenario.
          
            The 4 corners of our rectangle are the corners in the hashmap
            that were passed/landed-on more than once.  The Hashmap value
            is the number of times we passed the corner.  As defined in our 
            struct Corner = position + direction (prior to a turn) 

            The number of times these corners in a rectanglular loop is checked
            with an arbitrary check value that is set in repeat_count variable.
                    ^^^^^^^^^
        */

        let repeat_count = 4u32;

        if self.four_corners.len() == 4 {
            if completes_loop(self, repeat_count) {
                self.reset();
                return true;
            } else {
                return false;
            }
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
        // - Empty spaces (clear path) '.' the guard can patrol.
        for (i, line) in input.iter().enumerate() {
            let cols: Vec<char> = line.chars().collect();
            let pos_y = i as i32;
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
    original_position: Position,
    original_direction: Direction,
    position: Position,
    direction: Direction,
    directions: VecDeque<Direction>,
    vector: (i8, i8)
}

impl Guard {
    fn init(position: Position, direction: Direction) -> Self {
        Self {
            original_position: position.clone(),
            original_direction: direction.clone(),
            position,
            directions: Direction::circular(),
            direction,
            vector: vector(&direction)
        }
    }

    fn reset(&mut self) {
        let mut circular = Direction::circular();
        while let Some(d) = circular.front() {
            if *d != self.original_direction {
                circular.rotate_left(1);
            } else {
                self.directions = circular;
                break;
            }
        }
        self.position = self.original_position;
        self.direction = self.original_direction; 
        self.vector = vector(&self.direction);
    }

    fn turn_right(&mut self) {
        self.directions.rotate_left(1);
        if let Some (new_direction) = self.directions.front() {
            self.direction = *new_direction;
            self.vector = vector(new_direction);
        }
    }

    fn move_forward(&mut self) {
        let new_position = self.position.peek_ahead(self.vector);
        self.position = new_position
    }

    fn facing_obstacle(&self, grid: &Grid) -> bool {
        match whats_ahead(&self.position, &self.direction, grid) {
            Facing::Obstacle => true,
            _ => false
        }
    }

    fn exiting_edge(&self, grid: &Grid) -> bool {
        match whats_ahead(&self.position, &self.direction, grid) {
            Facing::Limbo => true,
            _ => false
        }
    }

    // For part 1
    fn patrolling(&mut self, grid: &Grid) -> HashSet<Position> {

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

    // For part 2
    fn looping(&mut self, grid: &Grid, patrol_path: &HashSet<Position>) -> HashSet<Position> {

        let mut obstruction_positions: HashSet<Position> = HashSet::new();
        let mut loop_corners: PatrolLoop = PatrolLoop::new();

        for path_pos in patrol_path.iter() {

            // According to puzzle we don't but obstacle on guard's
            // position. he/she will see us.
            if path_pos.same(&grid.starting) {
                continue;
            }

            // Copy the original grid and add the patrol path position 
            // as an obstacle then detect for infinite loop
            let mut mod_grid = grid.clone();
            mod_grid.obstacles.insert(*path_pos);

            // Reset guard position to original grid position
            self.reset();

            //println!("--- ADDED OBSTACLE ---\n{:?}", path_pos);
            //println!("--- NEW GRID ---\n{:?}", &mod_grid);
            //println!("--- GUARD STARTING ---\n{:?}", &self.position);

            // Set an abitrary count to exit our loop if obstruction 
            // is placed on current path_pos and we loop infinitely
            //let max_movement = u32::MAX;
            let max_movement = patrol_path.len() as u32 * 5;
            let mut move_count = 0u32;

            loop {
                if self.exiting_edge(&mod_grid) {
                    loop_corners.reset();
                    break;
                }
    
                // TODO: Remove, for debugging
                //println!("\n>>> guard is at position: {:?} facing: {:?}", self.position, self.direction);
                //println!(" >> corners count:");
                //let _ =  &loop_corners.corners_count.iter().for_each(|x| println!("  > {:?}", x));
                //println!(" >> four corners:");
                //let _ =  &loop_corners.four_corners.iter().for_each(|x| println!("  > {:?}", x));
    
                if loop_corners.check_loop() {
                    obstruction_positions.insert(*path_pos);
                    loop_corners.reset();
                    break;
                }
    
                if self.facing_obstacle(&mod_grid) {
                    loop_corners.add_corner((self.position.clone(), self.direction));
                    self.turn_right();
                } else {
                    self.move_forward();
                }

                // Detect an infinite loop and exit
                if move_count < max_movement {
                    move_count += 1;
                } else {
                    // if we reach max, assume we have found possible obstruction position
                    obstruction_positions.insert(*path_pos);
                    loop_corners.reset();
                    break;
                }
            }
        }
 
        // TODO: Remove for debugging
        //println!("\n>>>>> Obstruction positions: <<<<<");
        //let _ =  obstruction_positions.iter().for_each(|x| println!("  > {:?}", x));

        obstruction_positions
    }
}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

fn completes_loop(patrol_loop: &PatrolLoop, check_count: u32) -> bool {
    let mut buff_four: Vec<u32> = vec![];
    for last_4 in patrol_loop.four_corners.iter() {
        if let Some(count) = patrol_loop.corners_count.get(last_4) {
            buff_four.push(*count);
        }
    }
    if buff_four.len() == 4 {
        if buff_four.iter().all(|x| *x >= check_count) {
            return true;
        } else {
            return false;
        }
    } else {
        false
    } 
}

fn whats_ahead(pos: &Position, dir: &Direction, grid: &Grid) -> Facing {
    let v = vector(dir);
    let peeked = pos.peek_ahead(v);
    if peeked.x < grid.min.x || peeked.y < grid.min.y || peeked.x > grid.max.x || peeked.y > grid.max.y {
        // Exiting the grid...outer space...Limbooooooo
        Facing::Limbo
    } else if grid.obstacles.contains(&peeked) {
        // Look Out!!
        Facing::Obstacle
    } else {
        // Clear path ahead
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

fn puzzle_solve1(data: &Vec<String>) -> Result<HashSet<Position>, String> {
    let grid: Grid = Grid::init(data);
    let mut guard = Guard::init(grid.starting, Direction::North);
    let patrol_path: HashSet<Position> = guard.patrolling(&grid);
    Ok(patrol_path)
}

fn puzzle_solve2(data: &Vec<String>, patrol_path: &HashSet<Position>) -> Result<HashSet<Position>, String> {
    let grid: Grid = Grid::init(data);
    let mut guard = Guard::init(grid.starting, Direction::North);
    let possible_obstructions: HashSet<Position> = guard.looping(&grid, patrol_path);
    Ok(possible_obstructions)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    println!("\n>>>>>>>>>>> Puzzle Day 06 <<<<<<<<<<\n");

    // ----------- INPUT DATA ----------- //

    //let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;


    // ----------- PART 1 --------------- //

    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------");

    let patrol_path = puzzle_solve1(&data)?;

    println!("  Part 1 Result: {:?}\n", patrol_path.len());


    // ----------- PART 2 --------------- //

    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------");

    let results = puzzle_solve2(&data, &patrol_path)?;
    println!("  Part 2 Result: {:?}\n", results.len());

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
        //assert_eq!(puzzle_solve1(&d)?, test_expected);
        let patrol_path = puzzle_solve1(&d)?;
        assert_eq!(patrol_path.len() as u32, test_expected);

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

        // Call puzzle_solve1 from first test to setup required data for puzzle 2 solution
        if let Ok(patrol_path) = puzzle_solve1(&d) {
            // Test our solution
            let result_list= puzzle_solve2(&d, &patrol_path)?;
            assert_eq!(result_list.len() as u32, test_expected);
        } else {
            eprint!("Failed to get patrol path!");
            assert!(false);
        }

        Ok(())
    }
}