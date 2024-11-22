/* *************************************************************************
   THIS IS BASED ON RUST PROGRAMMING LANGUAGE "THE BOOK"
      > https://doc.rust-lang.org/book/, 
        Chapter 12 "Building a Command Line Program"
   ************************************************************************* */


/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::env;
use std::error::Error;
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
   pub file_input: String
}

impl PuzzleInput {
   fn init() -> Result<PuzzleInput, &'static str> {
      let args: Vec<String> = env::args().collect();
      if args.len() < 2 {
         return Err("not enough arguments");
      }

      let file_input= args[1].clone();

      Ok(PuzzleInput { file_input })
   }

   pub fn file_handle() -> Result<File, String> {
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
      match PuzzleInput::file_handle() {
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
}


/* *************************************************************************
                              HELPER FUNCTIONS
   ************************************************************************* */
