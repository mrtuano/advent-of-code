
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Concat
}


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
struct Equation {
    line: String,
    value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn new(line: &String) -> Result<Equation, String> {
        let parts: Vec<&str> = line.split(':').collect();
        let value = safe_parse(parts[0])?;
        let mut numbers: Vec<u64> = vec![];
        for n in parts[1].split_whitespace().into_iter() {
            match safe_parse(n) {
                Ok(r) => numbers.push(r),
                Err(e) => return Err(e)
            };
        }
        Ok(Equation {
            line: line.to_string(),
            value,
            numbers 
        })
    }

    fn verify(&self, operators: Vec<Operation>) -> bool {
        println!("\nVerifying: {:?}", &self.line);
        let mut op_results: Vec<bool> = vec![];
        for ops in generate_operators(self.numbers.len() - 1, operators) {
            let is_valid = match perform_operations(&self.numbers, &ops) {
                Some(result) => {
                    if result == self.value {
                        true
                    } else {
                        false
                    }
                },
                None => false
            };
            op_results.push(is_valid);
        }
        op_results.iter().any(|x|*x)
    }
}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn perform_operations(numbers: &Vec<u64>, operations: &Vec<Operation>) -> Option<u64> {
    if numbers.len() < 2 || operations.is_empty() {
        return Some(numbers[0]);
    }

    let first_operator = operations[0];
    let result = match first_operator {
        Operation::Add => numbers[0].checked_add(numbers[1]),
        Operation::Multiply => numbers[0].checked_mul(numbers[1]),
        Operation::Concat => safe_concat(&numbers[0].to_string(), &numbers[1].to_string())
    };

    match result {
        Some(val) => {
            let mut new_numbers = vec![val];
            new_numbers.extend_from_slice(&numbers[2..]);
            perform_operations(&new_numbers, &operations[1..].to_vec())
        },
        None => None
    }
}

fn safe_parse(s: &str) -> Result<u64, String> {
    if let Ok(n) = s.to_string().parse::<u64>() {
        Ok(n)
    } else {
        Err(format!("cannot parse string {:?}", s))
    }
}

fn safe_concat(s1: &str, s2: &str) -> Option<u64> {
    if let Ok(n) = safe_parse(&format!("{}{}", s1, s2)) {
        Some(n)
    } else {
        None
    }
}

/*fn generate_operators(length: usize, elements: Vec<Operation>) -> Vec<Vec<Operation>> {
    let mut results: Vec<Vec<Operation>> = Vec::new();
    let mut combination = vec![elements[0]; length];
    
    loop {
        results.push(combination.clone());

        let mut i = length - 1;
        while i > 0 && combination[i] == elements[1] {
            combination[i] = elements[0];
            i -= 1;
        }
        
        if combination[i] == elements[1] && i == 0 {
            break;
        }
        
        combination[i] = elements[1];
    }
    
    results
}*/

fn generate_operators(length: usize, elements: Vec<Operation>) -> Vec<Vec<Operation>> {
    let mut results = Vec::new();
    let mut combination = vec![elements[0]; length];
    backtrack(&mut results, &mut combination, &elements, length, 0);
    results
}

fn backtrack(results: &mut Vec<Vec<Operation>>, combination: &mut Vec<Operation>, 
    elements: &Vec<Operation>, length: usize, index: usize) {

    if index == length {
        results.push(combination.clone());
        return;
    }

    for &element in elements {
        combination[index] = element;
        backtrack(results, combination, elements, length, index + 1);
    }
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {
    let mut equations: Vec<Equation> = vec![];
    for line in data.iter() {
        match Equation::new(line) {
            Ok(q) => equations.push(q),
            Err(e) => return Err(e)
        }
    }
    
    let ops: Vec<Operation> = vec![
        Operation::Add, 
        Operation::Multiply
    ];

    let mut values: Vec<u64> = vec![];
    for q in equations.iter() {
        if q.verify(ops.clone()) {
            println!("Equation is true: {:?}", q.line);
            values.push(q.value);
        } else {
            println!("Equation is false: {:?}", q.line);
        }
    }

    Ok(values.iter().sum())
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u64, String> {
    let mut equations: Vec<Equation> = vec![];
    for line in data.iter() {
        match Equation::new(line) {
            Ok(q) => equations.push(q),
            Err(e) => return Err(e)
        }
    }
    
    let ops: Vec<Operation> = vec![
        Operation::Add, 
        Operation::Multiply, 
        Operation::Concat
    ];

    let mut values: Vec<u64> = vec![];
    for q in equations.iter() {
        if q.verify(ops.clone()) {
            println!("Equation is true: {:?}", q.line);
            values.push(q.value);
        } else {
            println!("Equation is false: {:?}", q.line);
        }
    }

    Ok(values.iter().sum())
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day XX <<<<<<<<<<\n");

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
        let test_expected = 3749u64;

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
        let test_expected = 11387u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}