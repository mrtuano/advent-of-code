
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::HashMap;

use aoc_utils::aoc_utils::*;

use petgraph::{algo::condensation, prelude::*, visit::IntoNodeReferences};


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
// ------------------------- Point Struct ------------------------- //
#[derive(Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn set(x: u32, y: u32) -> Self {
        Self {x, y}
    }

    fn sides(&self, max: &Point) -> Vec<Point> {
        let mut sides: Vec<Point> = vec![];
        if let Some(north) = self.north() {
            sides.push(north);
        }
        if let Some(east) = self.east(max) {
            sides.push(east);
        }
        if let Some(south) = self.south(max) {
            sides.push(south);
        }
        if let Some(west) = self.west() {
            sides.push(west);
        }
        sides
    }

    fn north(&self) -> Option<Point> {
        if let Some(y) = self.y.checked_sub(1) {
            Some(Point::set(self.x, y))
        } else {
            None
        }
    }
    
    fn east(&self, max: &Point) -> Option<Point> {
        if let Some(x) = self.x.checked_add(1) {
            if x <= max.x {
                Some(Point::set(x, self.y))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn south(&self, max: &Point) -> Option<Point> {
        if let Some(y) = self.y.checked_add(1) {
            if y <= max.y {
                Some(Point::set(self.x, y))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn west(&self) -> Option<Point> {
        if let Some(x) = self.x.checked_sub(1) {
            Some(Point::set(x, self.y))
        } else {
            None
        }
    }
    
    // ---------- Internal diagonals for Part 2 --------- //

    fn northwest(&self, _max: &Point) -> Option<Point> {
        let x = self.x.checked_sub(1)?;
        let y = self.y.checked_sub(1)?;
        Some(Point::set(x, y))
    }

    fn northeast(&self, max: &Point) -> Option<Point> {
        let x = self.x.checked_add(1)?;
        let y = self.y.checked_sub(1)?;
        if x <= max.x {
            Some(Point::set(x, y))
        } else {
            None
        }
    }

    fn southwest(&self, max: &Point) -> Option<Point> {
        let x = self.x.checked_sub(1)?;
        let y = self.y.checked_add(1)?;
        if y <= max.y {
            Some(Point::set(x, y))
        } else {
            None
        }
    }

    fn southeast(&self, max: &Point) -> Option<Point> {
        let x = self.x.checked_add(1)?;
        let y = self.y.checked_add(1)?;
        if x <= max.x && y <= max.y {
            Some(Point::set(x, y))
        } else {
            None
        }
    }

}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

// ----------------------------------------------------------------------
//
//  Graphing solution based on 
//       https://www.youtube.com/watch?v=uToJiPDp22M
//
// ----------------------------------------------------------------------


// ----------------------------------------------------------------------
// Shortened by AI (MS Copilot).  See next 4 functions
// ----------------------------------------------------------------------
/*fn northwest(node: &Point, grid: &HashMap<Point, char>, plant: &char, max: &Point) -> bool {

    let n = node.north().is_some_and(|p|{
        if let Some(x) = grid.get(&p) {
            x == plant
        } else {
            false
        }
    });

    let w = node.west().is_some_and(|p|{
        if let Some(x) = grid.get(&p) {
            x == plant
        } else {
            false
        }
    });

    if !n && !w {
        // Is an external North-West corner
        true
    } else if n && w {
        // Get the interior northwest diagonal of the node
        // if plant/char is not similar to node plant/char
        if let Some(se) = node.northwest(max) {
            if let Some(x) = grid.get(&se) {
                x == plant
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}*/

// ----------------------------------------------------------------------
// Next four functions are based on AI generated code.  They are shortened 
// version of above commented-out function (with minor differences as applicable).
// ----------------------------------------------------------------------

fn northwest(node: &Point, grid: &HashMap<Point, char>, plant: &char, max: &Point) -> bool {

    let n = node.north().and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);
    let w = node.west().and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);

    match (n, w) {
        (false, false) => true, // exterior corner
        (true, true) => { // interior corner
            node.northwest(max)
                .and_then(|p| grid.get(&p))
                .map_or(false, |&x| x != *plant)
        },
        _ => false,
    }
}

fn southwest(node: &Point, grid: &HashMap<Point, char>, plant: &char, max: &Point) -> bool {
    let s = node.south(max).and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);
    let w = node.west().and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);

    match (s, w) {
        (false, false) => true, // exterior corner
        (true, true) => { // interior corner opposite south-west diagonally
            node.southwest(max)
                .and_then(|p| grid.get(&p))
                .map_or(false, |&x| x != *plant)
        },
        _ => false,
    }
}

fn northeast(node: &Point, grid: &HashMap<Point, char>, plant: &char, max: &Point) -> bool {
    let n = node.north().and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);
    let e = node.east(max).and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);

    match (n, e) {
        (false, false) => true, // exterior corner
        (true, true) => { // interior corner opposite north-east diagonally
            node.northeast(max)
                .and_then(|p| grid.get(&p))
                .map_or(false, |&x| x != *plant)
        },
        _ => false,
    }
}

fn southeast(node: &Point, grid: &HashMap<Point, char>, plant: &char, max: &Point) -> bool {
    let s = node.south(max).and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);
    let e = node.east(max).and_then(|p| grid.get(&p)).map_or(false, |&x| x == *plant);

    match (s, e) {
        (false, false) => true, // exterior corner
        (true, true) => { // interior corner opposite south-east diagonally
            node.southeast(max)
                .and_then(|p| grid.get(&p))
                .map_or(false, |&x| x != *plant)
        },
        _ => false,
    }
}

fn corner_count(node: &Point, grid: &HashMap<Point, char>, max: &Point, plant: &char) -> Result<usize, String> {

    let mut count = 0usize;

    if southeast(node, grid, plant, max) {
        count += 1;
    } 

    if northeast(node, grid, plant, max) {
        count += 1;
    }

    if northwest(node, grid, plant, max) {
        count += 1;
    }

    if southwest(node, grid, plant, max) {
        count += 1;
    }

    Ok(count)
}

fn read_data(data: &Vec<String>) -> Result<
        (HashMap<Point, char>, UnGraphMap<Point, ()>, Graph<Vec<Point>, (), Undirected, NodeIndex>, Point), String> {

    let max: Point = Point::set(
        (data[0].chars().count() - 1) as u32,
        (data.len() - 1) as u32
    );

    let grid: HashMap<Point, char> = data.iter()
        .enumerate()
        .flat_map(|(y, line)|{
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::set(x as u32, y as u32), c))
            })
        .collect();

    let mut garden: UnGraphMap<Point, ()> = UnGraphMap::new();
    for (p, c) in grid.iter() {
        let node = garden.add_node(*p);
        for p1 in node.sides(&max).iter() 
            .filter(|x| grid.get(*x).map_or(false,|m| m == c)) {
            garden.add_edge(node, *p1, ());
        }
    }

    let regions = condensation(garden.clone().into_graph::<NodeIndex>(), false);

    Ok((grid, garden, regions, max))
}

// ----------------------- Puzzle Part 1 ---------------------- //
fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {

    let (_grid, garden, regions, _max) = read_data(data)?;

    let total_amount = regions.node_references()
        .map(|(_, region_points)|{
            let area = region_points.len() as u64;
            let perim = region_points.iter()
                .map(|region_point| 4 - garden.neighbors(*region_point).count())
                .sum::<usize>();
            area * perim as u64
        })
        .sum();

    Ok(total_amount)
}

// ----------------------- Puzzle Part 2 ---------------------- //
fn puzzle_solve2(data: &Vec<String>) -> Result<u64, String> {

    let (grid, _garden, regions, max) = read_data(data)?;

    let total_amount = regions.node_references()
        .map(|(_, region_points)|{
            let mut area = 0u64;
            let mut perim = 0usize;
            if let Some(plant) = grid.get(&region_points[0]) {
                area = region_points.len() as u64;
                perim = region_points.iter()
                    .map(|region_point| {
                        if let Ok(c) = corner_count(region_point, &grid, &max, &plant) {
                            c
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                }
                area * perim as u64
            }
        )
        .sum();

    Ok(total_amount)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 12 <<<<<<<<<<\n");

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
        let test_expected = 1930u64;

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
        let test_expected = 1206u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}