/* *************************************************************************
   THIS IS BASED ON RUST PROGRAMMING LANGUAGE "THE BOOK"
      > https://doc.rust-lang.org/book/, 
        Chapter 12 "Building a Command Line Program"
   ************************************************************************* */


/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


/* *************************************************************************
                              TRAITS
   ************************************************************************* */


/* *************************************************************************
                              ENUM AND METHODS
   ************************************************************************* */


/* *************************************************************************
                              STRUCTURE AND METHODS
   ************************************************************************* */
pub struct PuzzleInput {
   file_input: String
}

impl PuzzleInput {

   pub fn init() -> Result<Self, &'static str> {
      let args: Vec<String> = env::args().collect();
      if args.len() < 2 {
         return Err("not enough arguments");
      }

      let file_input= args[1].clone();

      Ok(PuzzleInput { file_input })
   }

   pub fn fh() -> Result<File, String> {
      match PuzzleInput::init() {
         Ok(p) => {
            if let Ok(f) = File::open(p.file_input.clone()) {
               return Ok(f);
            } else {
               return Err(format!("cannot open file {:?}", p.file_input.clone()));
            }
         },
         Err(e) => Err(e.to_string())
      }
   }

   pub fn vectorized() -> Result<Vec<String>, String> {
      match PuzzleInput::fh() {
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

   pub fn bufferized() -> Result<BufReader<File>, String> {
      match PuzzleInput::fh() {
         Ok(fh) => Ok(BufReader::new(fh)),
         Err(e) => Err(e)
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
   use super::*;

   #[test]
   fn missing_input() {
      let p = PuzzleInput::init();
      assert_eq!(p.is_err(), true);
   }

   #[test]
   fn invalid_input() {
      let fh = PuzzleInput::fh();
      assert_eq!(fh.is_err(), true);
   }

   // TODO: write more tests!!
}

