
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::{HashMap, VecDeque};

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

// ----------------------------------------------
// Point coordinates in the Grid
// ----------------------------------------------
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn set(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn point_up(&self) -> Option<Point> {
        if let Some(dy) = self.y.checked_add_signed(-1) {
            Some(Point::set(self.x, dy))
        } else {
            None
        }
    }

    fn point_down(&self, max: &Point) -> Option<Point> {
        if let Some(dy) = self.y.checked_add_signed(1) {
            if dy <= max.y {
                Some(Point::set(self.x, dy))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn point_right(&self, max: &Point) -> Option<Point> {
        if let Some(dx) = self.x.checked_add_signed(1) {
            if dx <= max.x {
                Some(Point::set(dx, self.y))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn point_left(&self) -> Option<Point> {
        if let Some(dx) = self.x.checked_add_signed(-1) {
            Some(Point::set(dx, self.y))
        } else {
            None
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


// ----------------------------------------------
// Grid i.e. Topographic map of the area
// ----------------------------------------------
#[derive(Debug)]
struct Grid {
    min: Point,
    max: Point,
    heights: HashMap<Point, u8>
}

impl Grid {
    fn init(data: &Vec<String>) -> Result<Grid, String> {

        let min: Point = Point::set(0, 0);
        let max: Point = Point::set(
            data[0].chars().count() as u32,
            data.iter().count() as u32
        );

        let mut heights: HashMap<Point, u8> = HashMap::new();
        for (i,l) in data.iter().enumerate() {
            let y = i as u32;
            for (j, h) in l.chars().enumerate() {
                let val = safe_parse8(&h.to_string())?;
                let x = j as u32;
                let p: Point = Point::set(x, y);
                heights.entry(p).or_insert(val);
            }
        }

        Ok(Grid { min, max, heights })
    }

    fn value_at(&self, x: u32, y: u32) -> Option<u8> {
        if let Some((_k, v)) = self.heights.get_key_value(&Point { x, y }) {
            Some(*v)
        } else {
            None
        }
    }

    fn trailheads(&self) -> Vec<Point> {
        let mut zeros: Vec<Point> = vec![];
        for y in 0..self.max.y {
            for x in 0..self.max.x {
                if let Some(v) = self.value_at(x, y) {
                    if v == 0 {
                        zeros.push(Point::set(x, y));
                    }
                }
            }
        }
        zeros
    }

    fn dump(&self) {
        println!("Grid Min: {:?}", self.min);
        println!("Grid Max: {:?}", self.max);
        println!("Grid Raw:");
        for y in 0..self.max.y {
            for x in 0..self.max.x {
                if let Some(v) = self.value_at(x, y) {
                    print!("{:?}", v);
                }
            }
            println!("");
        }
        println!("");
    }

}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn move_up(point: &Point, grid: &Grid, height: u8) -> Option<Point> {
    if let Some(p) = point.point_up() {
        if let Some(val) = grid.value_at(p.x, p.y) {
            if val == height {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn move_down(point: &Point, grid: &Grid, height: u8) -> Option<Point> {
    if let Some(p) = point.point_down(&grid.max) {
        if let Some(val) = grid.value_at(p.x, p.y) {
            if val == height {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn move_right(point: &Point, grid: &Grid, height: u8) -> Option<Point> {
    if let Some(p) = point.point_right(&grid.max) {
        if let Some(val) = grid.value_at(p.x, p.y) {
            if val == height {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn move_left(point: &Point, grid: &Grid, height: u8) -> Option<Point> {
    if let Some(p) = point.point_left() {
        if let Some(val) = grid.value_at(p.x, p.y) {
            if val == height {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

// Hmmm..., sounds like BFS algorithm...
fn trailhead_score(trailhead: &Point, grid: &Grid) -> u32 {
    let mut trail: VecDeque<Point> = VecDeque::new();
    let mut score = 0u32;

    trail.push_back(trailhead.clone());
    let mut height = 0u8;
    while let Some(p) = trail.pop_front() {
        if let Some(up) = move_up(&p, grid, height) {
            trail.push_back(up);
        }
        if let Some(up) = p.point_up() {
            if let Some(val) = grid.value_at(up.x, up.y) {
                if val == previous + 1 && val < 9 {
                    previous += 1;
                    trail.push_back(p);
                } else if val == previous + 1 && val == 9 {
                    score += 1;
                }
            }
        }
    }

    score
}


fn puzzle_solve1(grid: &Grid) -> Result<u32, String> {
    let trailheads = grid.trailheads();
    let mut scores: Vec<u32> = vec![];
    for t in trailheads.iter() {
        let s = trailhead_score(t, &grid);
        scores.push(s);
    }
    Ok(scores.iter().sum()) 
}

fn puzzle_solve2(grid: &Grid) -> Result<u32, String> {
    Ok(0)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 10 <<<<<<<<<<\n");

    let grid = Grid::init(&data)?;

    // TODO: Remove for testing only
    grid.dump();

    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------\n");
    println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&grid)?);

    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------\n");
    println!("  Part 2 Result: {:?}\n\n", puzzle_solve2(&grid)?);

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
        let test_expected = 36u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Interpret input data
        let grid = Grid::init(&d)?;
        grid.dump(); // For debugging

        // Test our solution
        assert_eq!(puzzle_solve1(&grid)?, test_expected);

        Ok(())
    }
   
    #[test]
    fn test_puzzle_solve2() -> Result<(), String> {

        // Update as needed
        let test_input = "test.data";
        let test_expected = 1u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Interpret input data
        let grid = Grid::init(&d)?;
        grid.dump(); // For debugging

        // Test our solution
        assert_eq!(puzzle_solve2(&grid)?, test_expected);

        Ok(())
    }
}