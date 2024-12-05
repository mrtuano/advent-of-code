
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::HashMap;

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */
#[derive(PartialEq, Eq)]
enum PuzzlePart {
    One,
    Two
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
#[derive(Debug)]
struct WordGrid {
    lines: Vec<String>,
    cols_lines: Vec<String>,
    grid: Vec<Vec<char>>
}

impl WordGrid {

    fn init(input: &Vec<String>) -> Result<WordGrid, String> {
        let mut grid: Vec<Vec<char>> = vec![];
        for l in input.iter() {
            let c: Vec<char> = l.chars().collect();
            grid.push(c);
        }

        Ok(WordGrid {
            lines: input.to_vec(),
            cols_lines: WordGrid::cols_to_rows(&grid),
            grid
        })
    }

    /*  
        Search word in all directions.
        The vector variables indicate the direction of search.
        Example, assuming x is the center in a grid...
            1  2  3
            4  x  5
            6  7  8
        the vector representation for each direction w.r.t. x,
            (-1, -1)  (-1, 0) (-1, 1)
            ( 0, -1)  ( 0, 0) ( 0, 1)
            ( 1, -1)  ( 1, 0) ( 1, 1)
        Hence,
            From x going diagonaly from cento to top-left, vector is: -1, -1
            From x going up vertically, vector is: -1, 0    
            From x going diagonally from center to top-right, vector is: -1, 1
            From x going left horizontally from center, vector is: 0, -1
            ...
            From x going bottm-right diagonally, vector is: 1, 1
    */
    fn word_search(&self, row: usize, col: usize, word: &str) -> Vec<Vec<(usize, usize)>> {

        // Part 2, return the set of coordinates of the complete word
        let mut word_coordinates: Vec<Vec<(usize, usize)>> = vec![];
        let mut buffer_coordinates: Vec<(usize, usize)> = vec![];

        let grid_max_x = self.grid.len() as isize;
        let grid_max_y = self.grid[0].len() as isize;

        let word_vec: Vec<char> = word.chars().collect();

        // first character of word check
        if self.grid[row][col] != word_vec[0] {
            return word_coordinates;
        }

        // vector variables vector_x and vector_y that indicates the direction of search
        // here we search only diagonally forward and backwards.
        let directions = 4usize;
        let vector_x: [i32; 4] = [-1, -1, 1, 1];
        let vector_y: [i32; 4] = [-1, 1, -1, 1];
        // to search all directions
        //let directions = 8usize;
        //let vector_x: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
        //let vector_y: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

        for arrow in 0..directions {
            let mut curr_x = (row as i32 + vector_x[arrow]) as isize;
            let mut curr_y = (col as i32 + vector_y[arrow]) as isize;
            // firt character matches, add to vec
            buffer_coordinates.push((row, col));
            // start at 2nd char of word for checking
            // check as we already checked first character above
            let mut k = 1usize;
            while k < word_vec.len() {
                // break if out of bounds
                if curr_x >= grid_max_x || curr_x < 0 || curr_y >= grid_max_y || curr_y < 0 {
                    break;
                }
                // break if chars do not match
                let g_x = curr_x as usize;
                let g_y = curr_y as usize;
                if self.grid[g_x][g_y] != word_vec[k] {
                    break;
                } else {
                    buffer_coordinates.push((g_x, g_y));
                }
                // move in the direction of our search vector_x, vector_y
                curr_x = (curr_x as i32 + vector_x[arrow]) as isize;
                curr_y = (curr_y as i32 + vector_y[arrow]) as isize;
                k += 1;
            }
            // if our word char index is length of word then word matched!
            if k == word_vec.len() {
                word_coordinates.push(buffer_coordinates.clone());
            }
            buffer_coordinates.clear();
        }
        word_coordinates
    }

    fn cols_to_rows(g: &Vec<Vec<char>>) -> Vec<String> {
        let mut new_lines: Vec<String> = vec![];
        for c in 0..g[0].len() {
            let mut new_row: Vec<char> = vec![];
            for line in g.iter() {
                new_row.push(line[c]);
            }
            let new_line: String = new_row.iter().collect();
            new_lines.push(new_line);
        }
        new_lines
    }

    fn word_occurrence_horizontally(&self, word: &str, reverse: bool) -> u32 {
        if !reverse {
            // left to right, then use matches method
            self.lines.iter().fold(0,|acc, x| {
                let y: Vec<&str> = x.matches(word).collect();
                acc + y.len() as u32
            })
        } else {
            // right to left, the word must be reversed then still use matches method
            let rword = String::from_iter(word.chars().rev());
            self.lines.iter().fold(0,|acc, x| {
                let y: Vec<&str> = x.matches(&rword).collect();
                acc + y.len() as u32
            })
        }
    }

    // Finding word vertically, we use the cols as rows vector and simply
    // use horizontal word search Rust matches 'pattern matching'.
    fn word_occurence_vertically(&self, word: &str, reverse: bool) -> u32 {
        if !reverse {
            // left to right, then use matches method
            self.cols_lines.iter().fold(0,|acc, x| {
                let y: Vec<&str> = x.matches(word).collect();
                acc + y.len() as u32
            })
        } else {
            // right to left, the word must be reversed then still use matches method
            let rword = String::from_iter(word.chars().rev());
            self.cols_lines.iter().fold(0,|acc, x| {
                let y: Vec<&str> = x.matches(&rword).collect();
                acc + y.len() as u32
            })
        }
    }

    /*
        Diagonal word search using grid.
        Could've used this same function for searching all directions
    */
    fn word_occurence_diagonally(&self, word: &str, puzzle_part: PuzzlePart)-> u32 {
        let mut word_occurences = 0u32;
        let mut word_occurences_coords: Vec<Vec<(usize, usize)>> = vec![];
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                let mut c = self.word_search(row, col, word);
                if c.len() > 0 {
                    word_occurences += c.len() as u32;
                    word_occurences_coords.append(&mut c);
                }
            }
        }
        if puzzle_part == PuzzlePart::One {
            return word_occurences;
        } else {
            // for part 2, we look for the intersecting coordinates and count
            // the number of pairs (i.e. similar coordinates)
            let mut intersecting_coordinates: HashMap<(usize, usize), u32> = HashMap::new(); 
            for w in word_occurences_coords.iter() {
                let x = intersecting_coordinates.entry(w[1]).or_insert(0);
                *x += 1;
            }
            println!(">> Part 2, intersecting coordinates:");
            let mut count = 0u32;
            for (_k, v) in intersecting_coordinates.iter() {
               if *v == 2 {
                    count += 1;
               } 
            }
            return count;
        }
    }
}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

fn puzzle_solve1(data: &Vec<String>, word: &str) -> Result<u32, String> {
    let wg: WordGrid = WordGrid::init(data)?;
    let l2r = wg.word_occurrence_horizontally(word, false);
    let r2l= wg.word_occurrence_horizontally(word, true);
    let vtb = wg.word_occurence_vertically(word, false);
    let vbt = wg.word_occurence_vertically(word, true);
    let dia = wg.word_occurence_diagonally(word, PuzzlePart::One);
    Ok(l2r + r2l + vtb + vbt + dia)
}

fn puzzle_solve2(data: &Vec<String>, word: &str) -> Result<u32, String> {
    let wg: WordGrid = WordGrid::init(data)?;
    let dia = wg.word_occurence_diagonally(word, PuzzlePart::Two);
    Ok(dia)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
fn main() -> Result<(), String> {

    let d = PuzzleInput::init(Some(&["this".to_string(), "input.data".to_string()]))?
        .vectorized()?;

    println!(">>> Solving Puzzle 4 Part 1:");
    let result = puzzle_solve1(&d, &"XMAS")?;
    println!("    XMAS occurences: {:?}", result);

    println!(">>> Solving Puzzle 3 Part 2:");
    let result = puzzle_solve2(&d, &"MAS")?;
    println!("    X-MAS occurences: {:?}", result);

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
        let d= PuzzleInput::init(Some(&["this".to_string(), "test.data".to_string()]))?
            .vectorized()?;
        let s = puzzle_solve1(&d, &"XMAS");
        assert_eq!(s, Ok(18u32));
        Ok(())
    }

    #[test]
    fn test_puzzle_solve2() -> Result<(), String> {
        let d= PuzzleInput::init(Some(&["this".to_string(), "test.data".to_string()]))?
            .vectorized()?;
        let s = puzzle_solve2(&d, &"MAS");
        assert_eq!(s, Ok(8u32));
        Ok(())
    }
}