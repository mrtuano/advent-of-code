
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

use aoc_utils::aoc_utils::*;


/* *************************************************************************
                            TRAITS
   ************************************************************************* */

/* *************************************************************************
                            CUSTOM TYPES
   ************************************************************************* */
type Page = u32;


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
#[derive(Clone, Debug)]
struct Update {
    pages: Vec<u32>
}

#[derive(Clone, Debug)]
struct Rules {
    page_number: Page,
    pages_after: HashSet<Page>,
}

impl Rules {
    fn new(page: Page) -> Self {
        Self {
            page_number: page,
            pages_after: HashSet::new(),
        }
    }

    fn check_rules(&self, page: Page, page_update: &Update, idx: usize) -> Option<bool> {
        // A little sanity check we are checking the right page
        if self.page_number != page {
            return None;
        }
        let pg_update_before: Vec<Page> = page_update.pages[0..idx].to_vec();
        // empty vector indicates the first entry in the update
        if pg_update_before.is_empty() {
            return Some(true);
        }
        for p in pg_update_before.iter() {
            if self.pages_after.contains(p) {
                return Some(false);
            } else {
                ();
            }
        }
        Some(true)
    }

}

// Part 2 solution:  Add the ordering and partial ordering traits
// for the Rule struct to get the expected page order and use
// it to derive weighted page.
impl Ord for Rules {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pages_after.contains(&other.page_number) {
            Ordering::Less
        } else if !self.pages_after.contains(&other.page_number) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Rules {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.pages_after.contains(&other.page_number) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialEq for Rules {
    fn eq(&self, other: &Self) -> bool {
        !self.pages_after.contains(&other.page_number)
    }
}

impl Eq for Rules {}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn stringify_error(e: ParseIntError) -> String {
    format!("cannot cast to u32\n{:?}", e)
}

fn init_rules(data: &Vec<String>) -> Result<HashMap<Page, Rules>, String> {

    // Map rules in a HashMap of Rule
    let mut page_rules: HashMap<Page, Rules> = HashMap::new();

    for line in data.iter() {
        if line.chars().any(|x|x == '|') {
            let v: Vec<&str> = line.trim().split('|').collect();
            if v.len() != 2 {
                continue;
            }
            let higher = v[0].parse::<Page>().map_err(stringify_error)?;
            let lower= v[1].parse::<Page>().map_err(stringify_error)?;
            let p = page_rules.entry(higher).or_insert(Rules::new(higher));
            p.pages_after.insert(lower);
        }
    }

    Ok(page_rules)
}

fn init_updates(data: &Vec<String>) -> Result<Vec<Update>, String> {

    let mut updates: Vec<Update> = vec![];
    let mut update_list: Vec<Page> = vec![];

    for line in data.iter() {
        if line.chars().any(|x|x == ',') {
            for n in line.trim().split(',') {
                let p: Page = n.parse::<Page>().map_err(stringify_error)?;
                update_list.push(p);
            }
            updates.push(Update {pages: update_list.clone()});
        }
        update_list.clear();
    }

    Ok(updates)
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u32, String> {

    let page_rules= init_rules(data)?;
    let updates = init_updates(data)?;
    let mut updates_in_right_order: Vec<Update> = vec![];

    for upt in updates.iter() {
        let mut is_right_order: bool = true;
        for (idx, page_in_update) in upt.pages.iter().enumerate() {
            if let Some(rules) = page_rules.get(&page_in_update) {
                if Some(false) == rules.check_rules(*page_in_update, upt, idx) {
                    is_right_order = false;
                    break;
                }
            }
        }
        if is_right_order {
            updates_in_right_order.push(upt.clone());
        }
    }

    let mut results = 0u32;
    for u in updates_in_right_order.iter() {
        let page_list = &u.pages;
        let middle_element = page_list.len() / 2;
        results += page_list[middle_element];
    }

    Ok(results)
}

// Part 2 solution...I got stuck over 24hrs !!! 
// ...searched the web and found someone solved it using Rust
//    as well.  Very elegant...short, simple and complete.
//    Still lots to learn...
fn puzzle_solve2(data: &Vec<String>) -> Result<u32, String> {

    // get rules from input data
    let mut rule_set: HashSet<(Page, Page)> = HashSet::new();
    for line in data.iter() {
        if line.chars().any(|x|x == '|') {
            let v: Vec<&str> = line.trim().split('|').collect();
            if v.len() != 2 {
                continue;
            }
            let higher = v[0].parse::<Page>().map_err(stringify_error)?;
            let lower= v[1].parse::<Page>().map_err(stringify_error)?;
            rule_set.insert((higher,lower));
        }
    }

    // parse update pages list from input data
    let mut part1 = 0u32;
    let mut part2 = 0u32;

    for line in data.iter() {

        if line.chars().any(|x|x == ',') {

            let mut pages: Vec<u32> = line.trim()
                .split(',')
                .map(|n| n.parse::<Page>().unwrap()) // I distain unwrap's but for now, ok.
                .collect();

            if pages.is_sorted_by(|a, b|!rule_set.contains(&(*b, *a))) {
                part1 += pages[pages.len() / 2];
            } else {
                pages.sort_by(|a, b| {
                    if rule_set.contains(&(*a, *b)) {
                        Ordering::Less
                    } else if rule_set.contains(&(*b, *a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                part2 += pages[pages.len() / 2];
            }
        }
    }

    // Print part 1 answer to verify our own solution
    println!("Part 1 answer: {:?}", part1);
    println!("Part 2 answer: {:?}", part2);
    Ok(part2)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 05 <<<<<<<<<<\n");

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
        let test_expected = 143u32;

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
        let test_expected = 123u32;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}