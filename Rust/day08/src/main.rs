
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::{HashMap, HashSet};

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
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
struct Position {
    x: isize,
    y: isize
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self {x, y}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Antenna {
    frequency: char,
    location: Position
}

impl Antenna {
    fn new(f: &char, p: &Position) -> Self {
        Self {
            frequency: *f,
            location: *p
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pair {
    first: Antenna,
    second: Antenna,
    dx: isize,
    dy: isize,
}

impl Pair {
    fn new(first: &Antenna, second: &Antenna) -> Option<Pair> {
        if first.frequency == second.frequency {
            let dx = first.location.x.abs_diff(second.location.x) as isize;
            let dy = first.location.y.abs_diff(second.location.y) as isize;
            // For part 1 ignore pairs that have a zero slope or 'infinite' slope
            if dx < 1 || dy < 1 {
                return None;
            } else {
                Some(Pair { 
                    first: first.clone(), 
                    second: second.clone(), 
                    dx: dx, 
                    dy: dy,
                })
            }
        } else {
            None
        }
    }

    fn nonresonant_antinodes(&self, grid: &Grid) -> Vec<Option<Position>> {
        // an element in returned vector with a None value is 
        // considered an antinode that is out-of-bounds (off the grid)
        vec![
            antinode(self.second.location, slope(&self.first.location, &self.second.location), grid),
            antinode(self.first.location, slope(&self.second.location, &self.first.location), grid),
        ]
    }

    fn resonant_antinodes(&self, grid: &Grid) -> Vec<Option<Position>> {
        let mut nodes: Vec<Option<Position>> = vec![
            Some(self.first.location), 
            Some(self.second.location)
        ];
        // antinodes on side of first antenna in pair of antennas
        let mut tilt: (isize, isize) = slope(&self.first.location, &self.second.location);
        let mut buffer_position = self.second.location;
        while let Some(p) = antinode(buffer_position, tilt, grid) {
            buffer_position = p;
            nodes.push(Some(p));
        }
        // antinodes on side of other antenna in pair of antennas
        tilt = slope(&self.second.location, &self.first.location);
        buffer_position = self.first.location;
        while let Some(p) = antinode(buffer_position, tilt, grid) {
            buffer_position = p;
            nodes.push(Some(p));
        }
        nodes
    }

} 

#[derive(Clone, Debug)]
struct Grid {
    min: Position,
    max: Position,
    antennas: HashSet<Antenna>
}

impl Grid {
    fn init(data: &Vec<String>) -> Result<Grid, String> {

        // Bounds of the Grid
        let min: Position = Position::new(0, 0);
        let max_x: isize = data[0].chars().count() as isize - 1;
        let max_y: isize = data.iter().len() as isize - 1;
        let max: Position = Position::new(max_x, max_y);

        // sanity check the grid
        if max_x < 1 || max_y < 1 {
            return Err("invalid grid!".to_string());
        }

        // Collect all antennas
        let mut antennas: HashSet<Antenna> = HashSet::new();
        for (y, line) in data.iter().enumerate() {
            for (x, thing) in line.chars().enumerate() {
                let p: Position = Position::new(x as isize, y as isize);
                if thing.is_ascii_alphanumeric() {
                    antennas.insert(Antenna::new(&thing, &p));
                }
            }
        }

        Ok(Grid {min, max, antennas})
    }

    fn antenna_pairs(&self) -> HashSet<Pair> {
        let mut pairs: HashSet<Pair> = HashSet::new();
        let antennas: Vec<&Antenna> = self.antennas.iter().collect();
        for i in 0..antennas.iter().len() - 1{
            if let Some(f) = antennas.get(i) {
                let mut j = i + 1;
                while j < self.antennas.iter().len() {
                    if let Some(s) = antennas.get(j) {
                        if let Some(pair )= Pair::new(f, s) {
                            pairs.insert(pair);
                        }
                    }
                    j += 1;
                }
            }
        }
        pairs
    }

}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn slope(first: &Position, second: &Position) -> (isize, isize) {
    (second.x - first.x, second.y - first.y)
}

fn antinode(point: Position, slope: (isize, isize), grid: &Grid) -> Option<Position> {
    let dx = point.x + slope.0;
    let dy = point.y + slope.1; 
    if dx >= grid.min.x && dy >= grid.min.y && dx <= grid.max.x && dy <= grid.max.y {
        Some(Position::new(dx, dy))
    } else {
        None
    }
}

fn puzzle_solve1(data: &Vec<String>) -> Result<usize, String> {

    // initialize the grid from input data
    let grid = Grid::init(data)?;

    // get the pairs of antennas
    let pairs: HashSet<Pair> = grid.antenna_pairs();

    // get the non-resonant harmonic antinodes for each pair of antenna
    let mut antinodes: HashMap<Pair, Vec<Option<Position>>> = HashMap::new();
    for pair in pairs.iter() {
        let mut nodes: Vec<Option<Position>> = pair.nonresonant_antinodes(&grid);
        antinodes.entry(pair.clone()).or_default().append(&mut nodes);
    }
    
    // create a list of all unique positions of antinodes
    let mut antinodes_vec: HashSet<Position> = HashSet::new();
    for (_p, n) in antinodes.iter() {
        let nodes: Vec<Position> = n.iter()
            .filter(|x|x.is_some())
            .map(|y| y.unwrap())
            .collect();

        nodes.iter().for_each(|x| {antinodes_vec.insert(*x);});
    }

    Ok(antinodes_vec.len())
}

fn puzzle_solve2(data: &Vec<String>) -> Result<usize, String> {

    // initialize the grid from input data
    let grid = Grid::init(data)?;

    // get the pairs of antennas
    let pairs: HashSet<Pair> = grid.antenna_pairs();

    // get the first all harmonic resonant antinodes for each pair of antenna
    let mut antinodes: HashMap<Pair, Vec<Option<Position>>> = HashMap::new();
    for pair in pairs.iter() {
        let mut nodes: Vec<Option<Position>> = pair.resonant_antinodes(&grid);
        antinodes.entry(pair.clone()).or_default().append(&mut nodes);
    }
    
    // create a list of all unique positions of all resonant harmonic antinodes
    let mut antinodes_vec: HashSet<Position> = HashSet::new();
    for (_p, n) in antinodes.iter() {
        let nodes: Vec<Position> = n.iter()
            .filter(|x|x.is_some())
            .map(|y| y.unwrap())
            .collect();

        nodes.iter().for_each(|x| {antinodes_vec.insert(*x);});
    }

    Ok(antinodes_vec.len())
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 08 <<<<<<<<<<\n");

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
        let test_expected = 14usize;

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
        let test_expected = 34usize;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}