
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

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

/* *************************************************************************
                            HELPER FUNCTIONS
   ************************************************************************* */



fn parse_input(d: &Vec<String>) -> Result<(Vec<u32>, Vec<u32>), String> {

   let mut left_list: Vec<u32> = vec![];
   let mut right_list: Vec<u32> = vec![];

   for i in d.iter() {
      let v: Vec<u32> = i.split_whitespace()
         .to_owned()
         .map(|x| {
            let n = match x.parse::<u32>() {
               Ok(y) => y,
               Err(e) => panic!("invalid input, entry not a number\n{:?}", e)
            };
            n
         })
         .collect();
      if v.len() != 2 {
         return Err("invalid input, not enough numbers".to_string());
      }
      left_list.push(v[0]);
      right_list.push(v[1]);
   }

   //left_list.sort();
   //left_list.dedup();
   //dbg!(&left_list.len());
   //right_list.sort();
   //right_list.dedup();
   //dbg!(&right_list.len());

   Ok((left_list, right_list))

}


fn puzzle_solve1(d: &Vec<String>) -> Result<u32, String> {
   let (mut left_list, mut right_list) = match parse_input(d) {
      Ok((l, r)) => (l, r),
      Err(e) => return Err(e)
   };
   left_list.sort();
   right_list.sort();
   let mut sum = 0u32;
   if left_list.len() != right_list.len() {
      return Err("not enough number pairs".to_string());
   }
   for i in 0..left_list.len() {
      let l = left_list.get(i).unwrap();
      let r = right_list.get(i).unwrap();
      sum += l.abs_diff(*r);
   }
   Ok(sum)
}

fn puzzle_solve2(d: &Vec<String>) -> Result<u32, String> {
   let (left_list, right_list) = match parse_input(d) {
      Ok((l, r)) => (l, r),
      Err(e) => return Err(e)
   };
   let mut similarity_score = 0u32;
   for n in left_list.iter() {
      let left_item = n.to_string();
      for r in right_list.iter() {
         let right_item = r.to_string().clone();
         let appeareances: Vec<&str> = left_item.matches(right_item.as_str()).collect();
         let item_score = n * appeareances.len() as u32;
         similarity_score += item_score;
      }
   }
   Ok(similarity_score)
}

/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
fn main() {
   match PuzzleInput::init(
      Some(&["this".to_string(), "input.data".to_string()])
   ) {
      Ok(p) => {
         let d = match p.vectorized() {
            Ok(z) => z,
            Err(_e) => vec![]
         };
         println!(">>> Solve for puzzle part 1:");
         let s = match puzzle_solve1(&d) {
            Ok(x) => x,
            Err(e) => panic!("Error! {:?}", e)
         };
         println!("Sum of difference: {:?}", s);

         println!(">>> Solve for puzzle part 2:");
         let s = match puzzle_solve2(&d) {
            Ok(x) => x,
            Err(e) => panic!("Error! {:?}", e)
         };
         println!("Similarity score: {:?}", s);
      },
      Err(e) => panic!("error!{:?}", e)
   };
}
    

/* *************************************************************************
                            TESTING
   ************************************************************************* */
#[cfg(test)]
mod tests {
   use super::*;
   //use aoc_utils::aoc_utils::*;

   #[test]
   fn test_example_data() {
      match PuzzleInput::init(
         Some(&["this".to_string(), "test.data".to_string()])
      ) {
         Ok(p) => {
            let d = match p.vectorized() {
               Ok(z) => z,
               Err(_e) => vec![]
            };
            assert_eq!(puzzle_solve1(&d), Ok(11u32));
            assert_eq!(puzzle_solve2(&d), Ok(31u32));
         },
         Err(_e) => assert!(false)
      };
   }
}