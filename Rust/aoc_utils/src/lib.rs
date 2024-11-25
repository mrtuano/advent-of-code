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
        pub file_input: String
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
            File::open(&self.file_input)
                .map_err(|_| format!("cannot open file {:?}", self.file_input))
        }
   
        pub fn vectorized(&self) -> Result<Vec<String>, String> {
            let fh = self.fh()?;
            BufReader::new(fh).lines()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("error while reading file: {:?}", e))
        }
   
        pub fn bufferized(&self) -> Result<BufReader<File>, String> {
            self.fh().map(BufReader::new)
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
        assert!(PuzzleInput::init(Some(&["this".to_string()])).is_err());
    }

    // First argument is considered to be the program name
    #[test]
    fn invalid_input() {
        assert!(PuzzleInput::init(Some(&["this".to_string(), "invalid_file_name".to_string()]))
            .map(|x| x.fh()).unwrap_or(Err("".to_string())).is_err());

    }

    #[test]
    fn valid_input() {
        assert!(
            PuzzleInput::init(Some(&["this".to_string(), "test_input.txt".to_string()]))
            .map(|x| x.fh())
            .unwrap_or(Err("".to_string())).is_ok()
        );
    }

    #[test]
    fn valid_vectorized() {
        assert!(
            PuzzleInput::init(Some(&["this".to_string(), "test_input.txt".to_string()]))
            .map(|x| x.vectorized())
            .unwrap_or(Err("".to_string())).is_ok()
        );
    }

    #[test]
    fn valid_bufferized() {
        assert!(
            PuzzleInput::init(Some(&["this".to_string(), "test_input.txt".to_string()]))
            .map(|x| x.bufferized())
            .unwrap_or(Err("".to_string())).is_ok()
        );
    }
    // TODO: write more tests!!

}

