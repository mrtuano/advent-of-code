/* *************************************************************************
    THIS IS BASED ON RUST PROGRAMMING LANGUAGE "THE BOOK"
        > https://doc.rust-lang.org/book/, 
        > Chapter 12 "Building a Command Line Program"
   ************************************************************************* */


/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */


/* *************************************************************************
                        TRAITS
   ************************************************************************* */


/* *************************************************************************
                        ENUM AND METHODS
   ************************************************************************* */


/* *************************************************************************
                        STRUCTURE AND METHODS
   ************************************************************************* */
pub mod aoc_utils {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub struct PuzzleInput {
        file_input: String
    }
   
    impl PuzzleInput {
   
        pub fn init(some_args: Option<&[String]>) -> Result<Self, &'static str> {

            let mut args: Vec<String> = std::env::args().collect();
            if some_args.is_some() {
                args = some_args.unwrap_or(&args).to_vec();
            }

            if args.len() < 2 {
                return Err("not enough arguments");
            }
   
            let file_input= args[1].clone();
   
            Ok(
                PuzzleInput { 
                    file_input 
                }
            )
        }
   
        pub fn fh(&self) -> Result<File, String> {
            if let Ok(f) = File::open(self.file_input.clone()) {
                return Ok(f);
            } else {
                return Err(format!("cannot open file {:?}", self.file_input));
            }
        }
   
        pub fn vectorized(&self) -> Result<Vec<String>, String> {
            match self.fh() {
                Ok(fh) => {
                    let mut lines: Vec<String> = vec![];
                    for l in BufReader::new(fh).lines() {
                        match l {
                            Ok(ln) => lines.push(ln.to_string()),
                            Err(e) => return Err(format!("error while reading file: {:?}", e))
                        };
                    }
                    Ok(lines)
                },
                Err(e) => Err(e)
            }
        }
   
        pub fn bufferized(&self) -> Result<BufReader<File>, String> {
            match self.fh() {
                Ok(fh) => Ok(BufReader::new(fh)),
                Err(e) => Err(e)
            }
        }
   
    }
}


/* *************************************************************************
                              HELPER FUNCTIONS
   ************************************************************************* */

/* *************************************************************************
                              TESTING
   ************************************************************************* */


#[cfg(test)]
mod tests {
    use crate::aoc_utils::PuzzleInput;

    // First argument is considered to be the program name
    #[test]
    fn missing_input() {
        let i: Vec<String> = vec!["one".to_string()];
        let p = PuzzleInput::init(Some(&i));
        assert_eq!(p.is_err(), true);
    }

    // First argument is considered to be the program name
    #[test]
    fn invalid_input() {
        let i: Vec<String> = vec![
            "one".to_string(),
            "invalid_file_name".to_string()
        ];
        if let Ok(p) = PuzzleInput::init(Some(&i)) {
            assert!(p.fh().is_err());
        } else {
            assert!(false);
        }
    }

    // TODO: write more tests!!
}

