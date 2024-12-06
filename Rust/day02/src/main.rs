
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
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Step {
    Up,
    Down,
    None
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
#[derive(PartialEq, Eq, Debug)]
struct Pair {
    first: u32,
    second: u32,
    diff: u32,
    step: Step
}

/* *************************************************************************
                            HELPER FUNCTIONS
   ************************************************************************* */

/*fn is_all_inc_dec(r: &Vec<u32>) -> bool {
    // Could've used is_sorted for the ascending order check
    // but this it to make it more verbose and obvious were doing 
    // descending and ascending order checks.
    /*if !r.is_sorted_by(|a, b| a <= b) && 
       !r.is_sorted_by(|a, b| a >= b) { 
        false
    } else {
        true
    }*/

    let mut prev_step: Step = Step::None;
    let mut report = r.clone();
    let mut pairs: Vec<Pair> = vec![];
    while let Some(g)= report.windows(2).next() {
        
        report.remove(0);
    }

    true
}

fn is_safe_diffs(r: &Vec<u32>) -> (bool, Vec<usize>) {
    let mut report = r.clone();
    let mut result: bool = true;
    let mut levels: Vec<usize> = vec![];
    let mut count = 0usize;
    while let Some(g)= report.windows(2).next() {
        if g.len() < 2 {
            result = false;
        }
        if g[0].abs_diff(g[1]) < 1 || g[0].abs_diff(g[1]) > 3 {
            result = false;
            levels.push(count);
        }
        report.remove(0);
        count += 1;
    }
    (result, levels)
}*/

fn parse_step(a: u32, b: u32) -> Step {
    if a > b {
        Step::Down
    } else if a < b {
        Step::Up
    } else {
        Step::None
    }
}

fn generate_pairs(r: &Vec<u32>) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = vec![];
    let mut report = r.clone();
    while let Some(g)= report.windows(2).next() {
        pairs.push(
            Pair {
                first: g[0],
                second: g[1],
                step: parse_step(g[0], g[1]),
                diff: g[0].abs_diff(g[1])
            }
        );
        report.remove(0);
    }
    pairs
}

fn safe_report1(r: &Vec<u32>) -> bool {

    //println!("report: {:?}", r);
    let pairs: Vec<Pair> = generate_pairs(r);

    let mut steps: HashMap<Step, u32> = HashMap::new();
    let mut unsafe_diffs = 0u32;

    for p in pairs.iter() {
        //println!("    pair: {:?}", &p);
        let s = steps.entry(p.step).or_insert(0);
        *s += 1;
        if p.diff < 1 || p.diff > 3 {
            unsafe_diffs += 1;
        }
    }

    if steps.keys().count() > 1 || unsafe_diffs > 0 {
        //println!("return FALSE");
        false
    } else {
        //println!("return TRUE");
        true
    }
}

/*fn safe_report(r: &Vec<u32>) -> bool {
    if r.is_empty() {
        return false;
    }
    if !is_all_inc_dec(r) || !is_safe_diffs(r).0 {
        return false;
    } else {
        return true;
    }
}*/

fn puzzle_solve1(data: &Vec<String>) -> Result<HashMap<usize, bool>, String> {
    let mut safe_reports: HashMap<usize, bool> = HashMap::new();
    for (i, line) in data.iter().enumerate() {
        let report: Vec<u32> = line.split_whitespace()
            .map(|x| {
                if let Ok(y) = x.parse::<u32>() {
                    Some(y)
                } else {
                    None
                }
            })
            .filter(|y| y.is_some())
            .map(|z|z.unwrap())
            .collect();
        safe_reports.entry(i).or_insert(safe_report1(&report));
    }
    Ok(safe_reports)
}

fn dampened_safe_report(r: &Vec<u32>) -> bool {

    // empty report
    if r.is_empty() {
        print!("\n\nreport: {:?} | is safe no dampener (empty report): FALSE", r);
        return false;
    }

    //print!("\n\nreport: {:?}", r);

    // no dampener needed, normal similar to puzzle 1 scenario
    if safe_report1(r) {
        print!("\n\nreport: {:?} | is safe no dampener: TRUE", r);
        return true;
    }


    // if we are here it means initial check failed
    // hence we apply dampener once and check
    let pairs: Vec<Pair> = generate_pairs(r);

    // identify the deficient increasing or decreasing level(s)
    let mut prev_step: Option<Step> = None;
    let mut deficient_levels: Vec<usize> = vec![];
    for (i, p) in pairs.iter().enumerate() {
        let i2 = i + 1;
        if p.diff < 1 || p.diff > 3 {
            deficient_levels.push(i);
        }
        if prev_step.is_none() {
            prev_step = Some(p.step.clone());
        } else if Some(p.step) != prev_step {
            deficient_levels.push(i);
        }
        prev_step = Some(p.step);
    }
    deficient_levels.sort();
    deficient_levels.dedup();

    // if deficient levels is not empty we damper it once by
    // removing of the deficient levels and check
    println!("    deficient levels: {:?}", &deficient_levels);
    if deficient_levels.len() > 0 {
        let e = deficient_levels[0];
        let mut report = r.clone();
        report.remove(e);
        if safe_report1(&report) {
            print!("\n\nreport: {:?} | is safe: TRUE (dampener applied)", r);
            println!("\n    dampened report: {:?}", &report);
            return true;
        } else {
            print!("\n\nreport: {:?} | is safe: FALSE (with dampener)", r);
            println!("\n    dampened report: {:?}", &report);
            return false;
        }
    } else {
        print!("\n\nreport: {:?} | is safe (post dampener): TRUE", r);
        return true;
    }

}

fn puzzle_solve2(data: &Vec<String>) -> Result<HashMap<usize, bool>, String> {
    let mut safe_reports: HashMap<usize, bool> = HashMap::new();
    for (i, line) in data.iter().enumerate() {
        let report: Vec<u32> = line.split_whitespace()
            .map(|x| {
                if let Ok(y) = x.parse::<u32>() {
                    Some(y)
                } else {
                    None
                }
            })
            .filter(|y| y.is_some())
            .map(|z|z.unwrap())
            .collect();
        let report_is_safe = dampened_safe_report(&report);
        safe_reports.entry(i).or_insert(report_is_safe);
    }
    Ok(safe_reports)
}

// Stuck on Part 2.  Search in SubReddit show some elegant Rust solutions
// This is based on one of them
fn is_safe_report_with_dampener(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        let inc = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<u32>>()
            .windows(2)
            .all(is_safe_increasing);
        let dec = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<u32>>()
            .windows(2)
            .all(is_safe_decreasing);

        if inc || dec {
            return true;
        }
    }
    false
}

fn is_safe_increasing(pair: &[u32]) -> bool {
    pair[1] > pair[0] && (pair[1] - pair[0] > 0) && (pair[1] - pair[0]) < 4
}

fn is_safe_decreasing(pair: &[u32]) -> bool {
    pair[0] > pair[1] && (pair[0] - pair[1] > 0) && (pair[0] - pair[1]) < 4
}
// up to here.

fn puzzle_solve2a(data: &Vec<String>) -> Result<HashMap<usize, bool>, String> {
    let mut safe_reports: HashMap<usize, bool> = HashMap::new();
    for (i, line) in data.iter().enumerate() {
        let report: Vec<u32> = line.split_whitespace()
            .map(|x| {
                if let Ok(y) = x.parse::<u32>() {
                    Some(y)
                } else {
                    None
                }
            })
            .filter(|y| y.is_some())
            .map(|z|z.unwrap())
            .collect();
        let report_is_safe = is_safe_report_with_dampener(&report);
        safe_reports.entry(i).or_insert(report_is_safe);
    }
    Ok(safe_reports)
}

/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
fn main() -> Result<(), String> {
    let d = PuzzleInput::init(Some(&["this".to_string(), "input.data".to_string()]))?
        .vectorized()?;

    println!(">>> Solving Puzzle 2 Part 1:");
    let safe_reports = puzzle_solve1(&d)?;
    let count_safe_reports = safe_reports.values().filter(|x| **x == true).count() as u32;
    println!("    Safe reports: {:?}", count_safe_reports);

    println!(">>> Solving Puzzle 2 Part 2:");
    //let new_safe_reports = puzzle_solve2(&d)?;
    let new_safe_reports = puzzle_solve2a(&d)?;
    let count_new_safe_reports = new_safe_reports.values().filter(|x| **x == true).count() as u32;
    println!("    New safe reports: {:?}", count_new_safe_reports);


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
        // get elements from test input
        let d= PuzzleInput::init(Some(&["this".to_string(), "test.data".to_string()]))?
            .vectorized()?;
        // puzzle part 1
        let s = puzzle_solve1(&d)?.values()
            .filter(|x| **x == true).count() as u32;
        assert_eq!(s, 2u32);
        Ok(())
    }

    #[test]
    fn test_puzzle_solve2() -> Result<(), String> {
        let d= PuzzleInput::init(Some(&["this".to_string(), "test.data".to_string()]))?
            .vectorized()?;
        // puzzle part 2
        //let s = puzzle_solve2(&d)?.values()
        let s = puzzle_solve2a(&d)?.values()
            .filter(|x| **x == true).count() as u32;
        assert_eq!(s, 4u32);
        Ok(())
    }
}