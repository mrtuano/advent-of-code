
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */


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
    fn word_search(&self, row: usize, col: usize, word: &str) -> u32 {

        let mut occurences = 0u32;

        let grid_max_x = self.grid.len() as isize;
        let grid_max_y = self.grid[0].len() as isize;

        let word_vec: Vec<char> = word.chars().collect();

        // first character of word check
        if self.grid[row][col] != word_vec[0] {
            return occurences;
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
                }
                // move in the direction of our search vector_x, vector_y
                curr_x = (curr_x as i32 + vector_x[arrow]) as isize;
                curr_y = (curr_y as i32 + vector_y[arrow]) as isize;
                k += 1;
            }
            // if our word char index is length of word then word matched!
            if k == word_vec.len() {
                occurences += 1;
            }
        }
        occurences
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
    fn word_occurence_diagonally(&self, word: &str) -> u32 {
        let mut word_occurences = 0u32;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                //if self.word_search(row, col, word) {
                //    println!("({:?}, {:?})", &row+1, &col+1);
                //    word_occurences += 1;
                //}
                word_occurences += self.word_search(row, col, word);
            }
        }
        word_occurences
    }

    /*
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // WARNING: Had to get help here, used AI to generate below method
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    fn word_occurence_diagonally_from_left(&self, word: &str) -> u32 {
        let word_len = word.len();
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let word_chars: Vec<char> = word.chars().collect();
        let mut count = 0u32;
    
        // Check top-left to bottom-right diagonals
        for i in 0..=rows.saturating_sub(word_len) {
            for j in 0..=cols.saturating_sub(word_len) {
                if (0..word_len).all(|k| self.grid[i + k][j + k] == word_chars[k]) {
                    //return true;
                    count += 1;
                }
            }
        }
    
        // Check bottom-left to top-right diagonals
        for i in word_len - 1..rows {
            for j in 0..=cols.saturating_sub(word_len) {
                if (0..word_len).all(|k| self.grid[i - k][j + k] == word_chars[k]) {
                    //return true;
                    count += 1;
                }
            }
        }
        count 
    }

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // WARNING: AI to generated code below method
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    fn word_occurence_diagonally_from_right(&self, word: &str) -> u32 {
        let word_len = word.len();
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let word_chars: Vec<char> = word.chars().rev().collect(); // Reverse the word
        let mut count = 0u32;
    
        // Check top-right to bottom-left diagonals
        for i in 0..=rows.saturating_sub(word_len) {
            for j in (word_len - 1..cols).rev() {
                if (0..word_len).all(|k| self.grid[i + k][j - k] == word_chars[k]) {
                    //return true;
                    count += 1;
                }
            }
        }
    
        // Check bottom-right to top-left diagonals
        for i in word_len - 1..rows {
            for j in (word_len - 1..cols).rev() {
                if (0..word_len).all(|k| self.grid[i - k][j - k] == word_chars[k]) {
                    //return true;
                    count += 1;
                }
            }
        }
    
        count
    }
    */
    


}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

fn puzzle_solve1(data: &Vec<String>, word: &str) -> Result<u32, String> {
    let wg: WordGrid = WordGrid::init(data)?;
    // TODO: Remove, only for debugging
    //println!("lines:");
    //let _ = &wg.lines.iter().for_each(|x|println!("{:?}", x));
    //println!("cols_lines:");
    //let _ = &wg.cols_lines.iter().for_each(|x|println!("{:?}", x));
    //println!("grid:");
    //let _ = &wg.grid.iter().for_each(|x|println!("{:?}", x));
    let l2r = wg.word_occurrence_horizontally(word, false);
    //dbg!(&l2r);
    let r2l= wg.word_occurrence_horizontally(word, true);
    //dbg!(&r2l);
    let vtb = wg.word_occurence_vertically(word, false);
    //dbg!(&vtb);
    let vbt = wg.word_occurence_vertically(word, true);
    //dbg!(&vbt);
    //let dlr = wg.word_occurence_diagonally_from_left(word);
    //dbg!(&dlr);
    //let drl = wg.word_occurence_diagonally_from_right(word);
    //dbg!(&drl);
    //let dia = dlr + drl;
    let dia = wg.word_occurence_diagonally(word);
    //dbg!(&dia);
    Ok(l2r + r2l + vtb + vbt + dia)
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u32, String> {
    Ok(0)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
fn main() -> Result<(), String> {

    let d = PuzzleInput::init(Some(&["this".to_string(), "input.data".to_string()]))?
        .vectorized()?;

    println!(">>> Solving Puzzle 4 Part 1:");
    let result = puzzle_solve1(&d, &"XMAS")?;
    println!("    Word occurences: {:?}", result);

    /*
    println!(">>> Solving Puzzle 3 Part 2:");
    let sum_products = puzzle_solve2(&d)?;
    println!("    Sum of products: {:?}", sum_products);
    */

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
        let s = puzzle_solve2(&d);
        assert_eq!(s, Ok(9u32));
        Ok(())
    }
}