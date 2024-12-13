
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::VecDeque;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ItemType {
    Filetype,
    Freespace
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

#[derive(Clone, Debug)]
struct DiskItem {
//    index: usize,
//    raw: char,
//    digit: u8,
//    blocks: u8,
    block_map: String,
//    file_id: Option<usize>,
//    item_type: ItemType
}

impl DiskItem {
//    fn new(index: usize, raw: &char, item_type: ItemType, 
//        file_id: Option<usize>) -> Option<DiskItem> {
    fn new(raw: &char, item_type: ItemType,
        file_id: Option<usize>) -> Option<DiskItem> {

        if let Ok(r) = raw_to_digit(&raw) {
            let block_map: String = draw_item(item_type, r, file_id).unwrap_or(String::new());
            Some(
                DiskItem {
//                    index,
//                    raw: *raw,
//                    digit: r,
//                    blocks: r,
//                    file_id,
//                    item_type,
                    block_map
                }
            )
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct DiskMap {
//    raw: String,
//    disk_items: Vec<DiskItem>,
    block_map: VecDeque<char>
}

impl DiskMap {

    fn init(data: &Vec<String>) -> Result<DiskMap, String> {

        //let mut disk_items: Vec<DiskItem> = vec![];
        let mut block_items: Vec<String> = vec![];

        let raw: String = data.concat();
        let mut file_id: usize = 0;

        for (index, char_item) in raw.chars().enumerate() {
            let item_type: ItemType = if index % 2 == 0 {
                ItemType::Filetype
            } else {
                ItemType::Freespace
            };
            //if let Some(disk_item) = DiskItem::new(index, &char_item, item_type, Some(file_id)) {
            if let Some(disk_item) = DiskItem::new(&char_item, item_type, Some(file_id)) {
                block_items.push(disk_item.block_map.clone());
                //disk_items.push(disk_item);
                if item_type == ItemType::Filetype {
                    file_id += 1;
                }
            } else {
                return Err(format!("cannot process item {:?} as DiskItem type", char_item));
            }
        }

        let block_map: VecDeque<char> = block_items.concat().chars().into_iter().collect();

        // TODO: Remove for debugging only.
        //       check the number of block_map if equal to number of chars in raw input.
        println!("Block map count: {:?}", block_map.iter().count());

        Ok(
            DiskMap {
//                raw,
//                disk_items,
                block_map
            }
        )

    }

    fn remap_blocks(&self) -> VecDeque<char> {

        let mut blockmap = self.block_map.clone();

        let mut front_ptr = 0usize;
        let mut back_ptr = blockmap.len() - 1;

        loop {
            if back_ptr <= front_ptr {
                break;
            }
            if !need_remap(&blockmap) {
                break;
            }
            let front_char = blockmap.get(front_ptr);
            let back_char = blockmap.get(back_ptr);
            if front_char.is_some() && back_char.is_some() {
                if front_char.unwrap().is_ascii_digit() {
                    front_ptr += 1;
                    continue;
                } 
                if !back_char.unwrap().is_ascii_digit() {
                    back_ptr -= 1;
                    continue;
                }
                blockmap.swap(front_ptr, back_ptr);
                front_ptr += 1;
                back_ptr -= 1;
            }
        }

        // TODO: Remove for debugging only
        let xx: String = blockmap.iter().collect();
        println!("Blockmap:\n\n{:?}", xx);

        blockmap
    }

}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn need_remap(blocks: &VecDeque<char>) -> bool {
    let block_str: String = blocks.iter().collect();

    // TODO: Remove, for testing
    //println!("Current: block map: {:?}", &block_str);

    let test_vector: Vec<&str> = block_str.split('.').into_iter().collect();
    if test_vector.len() > 1 {
        if test_vector[1..].to_vec().iter().any(|x|x.chars().any(|y| y.is_ascii_digit())) {
            return true;
        } else {
            return false;
        }
    }
    false
}

fn draw_item(item_type: ItemType, blocks: u8, file_id: Option<usize>) -> Option<String> {
    let mut draw_result: Option<String> = None;
    if item_type == ItemType::Freespace {
        let result: String = std::iter::repeat('.').take(blocks as usize).collect();
        draw_result = Some(result);
    } else {
        if let Some(x) = file_id {
            let result: String = std::iter::repeat(x.to_string()).take(blocks as usize).collect();
            draw_result = Some(result);
        }
    }
    draw_result
}

fn raw_to_digit(c: &char) -> Result<u8, String> {
    if c.is_ascii_digit() {
        match c.to_string().parse::<u8>() {
            Ok(x) => Ok(x),
            Err(e) => 
                return Err(format!("error parsing character '{:?}' as digit\n{:?}", *c, e))
        }
    } else {
        Err(format!("cannot parse {:?} as a number!", *c))
    }
}

fn filesystem_checksum(v: VecDeque<char>) -> Result<u64, String> {
    let mut check_sum = 0u64;
    for (i, x ) in v.iter().enumerate() {
        if *x == '.' {
            break;
        }
        let num = safe_parse(&x.to_string())?;
        check_sum += num * i as u64;
    }
    Ok(check_sum)
}

fn safe_parse(s: &str) -> Result<u64, String> {
    if let Ok(n) = s.to_string().parse::<u64>() {
        Ok(n)
    } else {
        Err(format!("cannot parse string {:?}", s))
    }
}

fn puzzle_solve1(data: &Vec<String>) -> Result<u64, String> {
    let disk_map = DiskMap::init(data)?;

    // TODO: Remove, for debugging only
    //let _ = disk_map.disk_items.iter()
    //    .for_each(|x| println!("index: {:?}, raw: {:?}, map: {:?}", x.index, x.raw, x.block_map));
    //println!("Block Map: {:?}", disk_map.block_map);

    filesystem_checksum(disk_map.remap_blocks())
}

fn puzzle_solve2(data: &Vec<String>) -> Result<u64, String> {
    todo!();
}


/* *************************************************************************
                            MAIN PROGRAM
   ************************************************************************* */
    fn main() -> Result<(), String> {

    // Update as needed
    let input_data = "input.data";

    let data = PuzzleInput::init(Some(&["this".to_string(), input_data.to_string()]))?
        .vectorized()?;

    println!("\n>>>>>>>>>>> Puzzle Day 09 <<<<<<<<<<\n");

    println!("---------------");
    println!("Solve Part 1:");
    println!("---------------\n");
    println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&data)?);

    println!("---------------");
    println!("Solve Part 2:");
    println!("---------------\n");
    println!("  Part 2 Result: {:?}\n\n", puzzle_solve2(&data)?);

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
        let test_expected = 1928u64;

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
        let test_expected = 0u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}