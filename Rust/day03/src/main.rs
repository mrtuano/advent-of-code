
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::u32;

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
struct Numbers {
    phrase: String,
    first: u32,
    second: u32,
    product: u32,
    include: bool
}

impl Numbers {
    fn new(p: &str, i: bool, x: &str, y: &str) -> Result<Numbers, String> {
        let num_x = match x.parse::<u32>() {
            Ok(z) => z,
            Err(e) => return Err(format!("{:?}", e))
        };
        let num_y = match y.parse::<u32>() {
            Ok(z) => z,
            Err(e) => return Err(format!("{:?}", e))
        };
        Ok(
            Numbers {
                first: num_x,
                second: num_y,
                phrase: p.to_string(),
                include: i,
                product: num_x * num_y
            }
        )
    }
}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

// Could've used string regexes here using an external create but..., 
// ...oh well...maybe later as an enhancement or extra work.
fn parse_input(d: &Vec<String>, p: bool) -> Result<Vec<Numbers>, String> {

    let mut numbers_list: Vec<Numbers> = vec![];
    let long_input = d.concat();
    
    // TODO: Remove, for debugging only
    //println!("long string:\n\n{:?}", long_input);

    let mut phrase: Vec<char> = vec![];
    let mut phrase_init: bool = false;
    let mut include_phrase: Vec<char> = vec![];
    let mut include_init: bool = true;
    let mut do_include: bool = true;

    for c in long_input.chars() {
        match c {
            // Part 2, get the do or don't
            'd' => {
                if !include_init {
                    include_phrase.clear();
                    include_phrase.push(c);
                    include_init = true;
                }
            },
            'o'|'n'|'\'' => {
                if include_init {
                    include_phrase.push(c);
                }
            },
            't' => {
                if include_init && include_phrase.len() == 4 {
                    if &include_phrase[0..] == ['d', 'o', 'n', '\''] {
                        do_include = false;
                    }
                }
            },
            // we only want contiguous mul(x,y) characters
            // if there are characters in between that pattern
            // reset the phrase vector in the catch-all pattern bellow
            'm' => {
                if !phrase_init {
                    phrase.clear();
                    phrase.push(c);
                    phrase_init = true;
                } else {
                    println!("here 1: {:?}", &c);
                }
            },
            'u'|'l'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|',' => {
                if phrase_init {
                    phrase.push(c);
                } else {
                    println!("here 2: {:?}", &c);
                }
            },
            // Separate for part 2
            '(' => {
                if phrase_init {
                    phrase.push(c);
                }
                if include_init {
                    include_phrase.push(c);
                }
            },
            ')' => {
                // TODO: A lot of assumptions here, fix them
                if &include_phrase[0..] == ['d', 'o', '('] {
                    do_include = true;
                    include_phrase.clear();
                    include_init = false;
                } else if &include_phrase[0..] == ['d', 'o', 'n', '\'', 't', '('] {
                    do_include = false;
                    include_phrase.clear();
                    include_init = false;
                }
                if phrase_init && phrase.len() >= 7 {
                    if &phrase[0..4] == ['m', 'u', 'l', '('] {
                        let body: &[char] = &phrase[4..];
                        let body_string: String = body.iter().collect();
                        phrase.push(')');
                        let phrase_string: String = phrase.iter().collect();
                        let numbers: Vec<&str>  = body_string.split(',').collect();
                        let mut consider_number = true;
                        if !p {
                            consider_number = do_include;
                        }
                        if numbers.len() == 2 {
                            let n = Numbers::new(&phrase_string, 
                                consider_number, numbers[0], numbers[1]
                            );
                            match n {
                                Ok(z) => numbers_list.push(z),
                                Err(e) => return Err(format!("{:?}", e))
                            };
                        } else {
                            println!("here 3: {:?}", &c);
                        }
                    } else {
                        println!("here 4: {:?}", &c);
                    }
                } else {
                    println!("here 5: {:?}", &c);
                }
                phrase_init = false;
            },
            _ => {
                println!("here 6: {:?}", &c);
                phrase_init = false
            }
        };
    }
    Ok(numbers_list)
}

fn puzzle_solve1(d: &Vec<String>) -> Result<u32, String> {

    let batches = match parse_input(d, false) {
        Ok(n) => n,
        Err(e) => return Err(format!("{:?}", e))
    };

    // TODO: Remove, for debugging
    for b in batches.iter() {
        println!(">>> b:\t{:?}", b);
    }

    let sum_products = &batches.iter()
        .fold(0, |acc, p| acc + p.product);

    Ok(*sum_products)
}

fn puzzle_solve2(d: &Vec<String>) -> Result<u32, String> {
    let batches = match parse_input(d, true) {
        Ok(n) => n,
        Err(e) => return Err(format!("{:?}", e))
    };

    // TODO: Remove, for debugging
    for b in batches.iter() {
        println!(">>> b:\t{:?}", b);
    }

    let sum_products = &batches.iter()
        .fold(0, |acc, p| acc + p.product);

    Ok(*sum_products)
}

/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */

fn main() -> Result<(), String> {

    let d = PuzzleInput::init(Some(&["this".to_string(), "input.data".to_string()]))?
        .vectorized()?;

    println!(">>> Solving Puzzle 3 Part 1:");
    let sum_products = puzzle_solve1(&d)?;
    println!("    Sum of products: {:?}", sum_products);

    /*
    println!(">>> Solving Puzzle 3 Part 2:");
    let new_safe_reports = puzzle_solve2(&d)?;
    let count_new_safe_reports = new_safe_reports.values().filter(|x| **x == true).count() as u32;
    println!("\n\nNew safe reports: {:?}", count_new_safe_reports);
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
        let s = puzzle_solve1(&d);
        assert_eq!(s, Ok(161u32));
        Ok(())
    }

    /*#[test]
    fn test_puzzle_solve2() -> Result<(), String> {
        let d= PuzzleInput::init(Some(&["this".to_string(), "test.data".to_string()]))?
            .vectorized()?;
        let s = puzzle_solve2(&d);
        assert_eq!(s, 48u32);
        Ok(())
    }*/
}