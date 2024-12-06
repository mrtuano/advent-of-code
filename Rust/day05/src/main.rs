
/* *************************************************************************
   NOTE: The code below is based on AI generated code for Priority Queues
         and Priority Graphs below.  Specifically Microsoft Copilot 
   ************************************************************************* */

/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
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
    index: usize,
    pages: Vec<u32>
}

impl Update {
    fn extract_weighted_pages(&self, pg: &BTreeMap<Page, u32>) -> Vec<Page> {
        let mut extracted: BinaryHeap<WeightedPage> = BinaryHeap::new();
        for page in self.pages.iter() {
            if let Some((wp, prio)) = pg.get_key_value(page) {
                extracted.push(WeightedPage::new(*wp, *prio));
            }
        }
        let expected: Vec<Page> = extracted.into_sorted_vec()
            .iter()
            .map(|z|z.page)
            .collect();
        expected
    }

    fn fix_page_order(&self, rules: &HashMap<Page, Rules>) -> Option<Update> {
        // TODO, implement this
        todo!();
    }
}

// ----------------------------------------------------------------------------
// NOTE: The struct definitions and impl code below is based on AI generated code 
//       for Priority Graphs and queues.  It has been tailored for solving 
//       the puzzle.
// ----------------------------------------------------------------------------
#[derive(Debug, Eq, PartialEq)]
struct WeightedPage {
    page: Page,
    priority: u32
}

impl Ord for WeightedPage {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for WeightedPage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl WeightedPage {
    fn new(page: Page, priority: u32) -> Self {
        Self {
            page,
            priority
        }
    }
}

#[derive(Debug)]
struct Rules {
    page_number: Page,
    pages_after: HashSet<Page>,
    //pages_before: HashSet<Page>
}

impl Rules {
    fn new(page: Page) -> Self {
        Self {
            page_number: page,
            pages_after: HashSet::new(),
            //pages_before: HashSet::new()
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

#[derive(Debug)]
struct PageGraph {
    rules: HashMap<Page, Vec<Page>>,
    weights: HashMap<Page, usize>
}

impl PageGraph {

    // Create a new priority graph
    fn init() -> Self {
        Self {
            rules: HashMap::new(),
            weights: HashMap::new()
        }
    }

    // Add rule to our priority graph
    fn add_rule(&mut self, higher: Page, lower: Page) {
        self.rules.entry(higher).or_default().push(lower.clone());
        *self.weights.entry(higher).or_insert(0) += 0;
        *self.weights.entry(lower).or_insert(0) += 1;
    }

    // return sorted list of pages based on all the parsed rules
    //fn sort_pages(&self) -> Result<BinaryHeap<WeightedPage>, &'static str> {
    fn sort_pages(&self) -> Result<BTreeMap<Page, u32>, &'static str> {
        let mut weights = self.weights.clone();
        let mut buffer_queue: VecDeque<Page> = weights.iter()
            .filter(|&(_, &w)| w == 0)
            .map(|(p, _)| *p)
            .collect();

        buffer_queue.clear();
        if let Some((bq, bs)) = weights.iter().min_by_key(|&(_, v)|v) {
            buffer_queue.push_back(*bq);
        }
        dbg!(&weights.len());
        dbg!(&buffer_queue);

        //let mut sorted_pages: BinaryHeap<WeightedPage> = BinaryHeap::new();
        let mut srted: BTreeMap<Page, u32> = BTreeMap::new();
        let mut priority = weights.len() as u32;

        while let Some(p) = buffer_queue.pop_front() {
            //sorted_pages.push(WeightedPage::new(p, priority));
            srted.insert(p, priority);
            priority -= 1;

            if let Some(neighbors) = self.rules.get(&p) {
                for neighbor in neighbors {
                    if let Some(d) = weights.get_mut(neighbor) {
                        *d -= 1;
                        if *d == 0 {
                            buffer_queue.push_back(*neighbor);
                        }
                    }
                }

            }
        }

        //if sorted_pages.len() == weights.len() {
        dbg!(&srted.keys().len());
        if srted.keys().len() == weights.len() {
            //Ok(sorted_pages)
            Ok(srted)
        } else {
            Err("Page graph has disconnected pages!")
        }
    }
}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn stringify_error(e: ParseIntError) -> String {
    format!("cannot cast to u32\n{:?}", e)
}

fn init_rules(data: &Vec<String>) -> Result<HashMap<Page, Rules>, String> {
    // Map the rules to a priority graph
    let mut pg: PageGraph = PageGraph::init();
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
            pg.add_rule(higher, lower); 
            let p = page_rules.entry(higher).or_insert(Rules::new(higher));
            p.pages_after.insert(lower);
        }
    }
    Ok(page_rules)
}

fn init_updates(data: &Vec<String>) -> Result<Vec<Update>, String> {
    let mut updates: Vec<Update> = vec![];
    let mut update_list: Vec<Page> = vec![];
    let mut idx = 0usize;
    for (_i, line) in data.iter().enumerate() {
        if line.chars().any(|x|x == ',') {
            for n in line.trim().split(',') {
                let p: Page = n.parse::<Page>().map_err(stringify_error)?;
                update_list.push(p);
            }
            updates.push(Update {index: idx, pages: update_list.clone()});
            idx += 1;
        }
        update_list.clear();
    }
    Ok(updates)
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u32, String> {
    let mut results = 0u32;
    let page_rules= init_rules(data)?;

    let updates = init_updates(data)?;
    let mut updates_in_right_order: Vec<Update> = vec![];
    for upt in updates.iter() {
        let mut is_right_order: bool = true;
        for (idx, page_in_update) in upt.pages.iter().enumerate() {
            if let Some(rules) = page_rules.get(&page_in_update) {
                if let Some(i) = rules.check_rules(*page_in_update, upt, idx) {
                    if !i {
                        is_right_order = i;
                        break;
                    }
                }
            }
        }
        if is_right_order {
            updates_in_right_order.push(upt.clone());
        }
    }
    for u in updates_in_right_order.iter() {
        let page_list = &u.pages;
        let middle_element = page_list.len() / 2;
        results += page_list[middle_element];
    }
    Ok(results)
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u32, String> {

    let page_rules= init_rules(data)?;
    let updates = init_updates(data)?;

    let mut updates_in_wrong_order: Vec<Update> = vec![];
    for upt in updates.iter() {
        let mut is_right_order: bool = true;
        for (idx, page_in_update) in upt.pages.iter().enumerate() {
            if let Some(rules) = page_rules.get(&page_in_update) {
                if let Some(i) = rules.check_rules(*page_in_update, upt, idx) {
                    if !i {
                        is_right_order = i;
                        break;
                    }
                }
            }
        }
        if !is_right_order {
            updates_in_wrong_order.push(upt.clone());
        }
    }

    let corrected_updates: Vec<Option<Update>> = updates_in_wrong_order.iter()
        .map(|u| u.fix_page_order(&page_rules))
        .collect();

    let mut results = 0u32;
    for u in corrected_updates.iter() {
        match u {
            Some(t) => {
                let page_list = &t.pages;
                let middle_element = page_list.len() / 2;
                results += page_list[middle_element];
            },
            None => continue
        };
    }
    Ok(results)
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!(">>>>>>>>>>> Puzzle Day 05 <<<<<<<<<<\n");

    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------");
    println!("  Part 1 Result: {:?}", puzzle_solve1(&data)?);

    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------");
    println!("  Part 2 Result: {:?}", puzzle_solve2(&data)?);

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