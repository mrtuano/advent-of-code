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

    // ------------------------------------------------------------------------
    // Exposed Structs and Methods
    // ------------------------------------------------------------------------

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


    // ------------------------------------------------------------------------
    // Exposed Functions
    // ------------------------------------------------------------------------


    pub fn safe_parse8(s: &str) -> Result<u8, String> {
        if let Ok(n) = s.to_string().parse::<u8>() {
            Ok(n)
        } else {
            Err(format!("cannot parse string {:?}", s))
        }
    }

    pub fn safe_parse32(s: &str) -> Result<u32, String> {
        if let Ok(n) = s.to_string().parse::<u32>() {
            Ok(n)
        } else {
            Err(format!("cannot parse string {:?}", s))
        }
    }

    pub fn safe_parse64(s: &str) -> Result<u64, String> {
        if let Ok(n) = s.to_string().parse::<u64>() {
            Ok(n)
        } else {
            Err(format!("cannot parse string {:?}", s))
        }
    }

    pub fn safe_parse(s: &str) -> Result<usize, String> {
        if let Ok(n) = s.to_string().parse::<usize>() {
            Ok(n)
        } else {
            Err(format!("cannot parse string {:?}", s))
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

    use crate::aoc_utils::*;

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

    #[test]
    fn test_safe_parse() {
        assert_eq!(
            safe_parse("55"), 
            Ok(55usize)
        );
    }

    #[test]
    fn test_safe_parse8() {
        assert_eq!(
            safe_parse8("9"), 
            Ok(9u8)
        );
    }

    #[test]
    fn test_safe_parse32() {
        assert_eq!(
            safe_parse32("17"), 
            Ok(17u32)
        );
    }

    #[test]
    fn test_safe_parse64() {
        assert_eq!(
            safe_parse64("28"), 
            Ok(28u64)
        );
    }

    // TODO: write more tests!!

}
