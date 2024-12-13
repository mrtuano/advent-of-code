
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::{HashMap, VecDeque};

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


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

#[derive(Clone, Copy, Debug)]
struct FileItem {
    id: Option<usize>,
    blocks: u8,
}

impl FileItem {
    fn new(raw: &char, id: Option<usize>) -> Result<FileItem, String> {
        let blocks = raw_to_digit(raw)?;
        Ok(FileItem { id, blocks })
    }
}

#[derive(Debug)]
struct DiskMap {
    raw: String,
    disk_items: Vec<FileItem>,
    block_items: HashMap<usize, FileItem>
}

impl DiskMap {

    fn init(data: &Vec<String>) -> Result<DiskMap, String> {

        let mut disk_items: Vec<FileItem> = vec![];
        let mut block_items: HashMap<usize, FileItem> = HashMap::new();

        let mut file_id: usize = 0;

        let raw: String = data.concat();

        for (index, char_item) in raw.chars().enumerate() {
            if index % 2 == 0 {
                let file_item = FileItem::new(&char_item, Some(file_id))?;
                file_id += 1;
                disk_items.push(file_item);
            } else {
                let file_item = FileItem::new(&char_item, None)?;
                disk_items.push(file_item);
            };
        }

        // TODO: Remove, for debugging only
        //println!("Disk Map:");
        //let _ = &disk_items.iter().for_each(|x| println!("    {:?}", x));

        Ok(DiskMap { raw, disk_items, block_items })
    }

    fn diskmap_to_blockmap(&self) -> VecDeque<Option<usize>> {
        let mut block_map: VecDeque<Option<usize>> = VecDeque::new();
        for b in self.disk_items.iter() {
            let mut blocks: VecDeque<Option<usize>> = map_to_blocks(b);
            block_map.append(&mut blocks);
        }
        block_map
    }

    fn diskmap_to_itemsmap(&self) -> VecDeque<(Option<usize>, u8)> {
        let mut items_map: VecDeque<(Option<usize>, u8)> = VecDeque::new();
        for b in self.disk_items.iter() {
            let mut items: VecDeque<(Option<usize>, u8)> = map_to_items(b);
            items_map.append(&mut items);
        }
        items_map
    }

}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

/*
fn remap_items(mut block_map: VecDeque<(usize, u8)>) -> VecDeque<Option<usize>> {

    let mut front_ptr = 0usize;
    let mut back_ptr = block_map.len() - 1;

    while back_ptr > front_ptr {
        if !need_block_remap(&block_map) {
            break;
        } else {
            let front_char = block_map.get(front_ptr);
            let back_char = block_map.get(back_ptr);
            if front_char.is_some() && back_char.is_some() {
                if front_char.unwrap().is_some() {
                    front_ptr += 1;
                    continue;
                } 
                if back_char.unwrap().is_none() {
                    back_ptr -= 1;
                    continue;
                }
                block_map.swap(front_ptr, back_ptr);
                front_ptr += 1;
                back_ptr -= 1;
            }
        }
    }

    let mut back_buffer = 0usize;
    let mut prev_back: Option<usize> = None;
    while let Some(back_char) = block_map.get(back_ptr) {
        if back_char.is_none() {
            back_ptr -= 1;
            prev_back = None;
            continue;
        } else if prev_back == None {
            prev_back == *back_char;
            back_buffer += 1;
            back_ptr -= 1;
        } else if prev_back == *back_char {
            back_buffer  += 1;
            back_ptr -= 1;
        } else if prev_back != None && prev_back != *back_char {
            let needed = back_buffer; // amount of freespace needed to move the file
            let start_back_ptr = back_ptr; // the pointer to the first block of the file (left to right)
            prev_back = *back_char; // for next iteration
            back_buffer = 0; // reset
            back_ptr -= 1;
            // at this point we wont move our back_ptr until we move the file
            // or not able to find a free space.
            if let Some(front_char) = block_map.get(front_ptr) {
                let mut front_pointers: Vec<usize> = vec![];
                if front_char.is_none() {
                    front_pointers.push(front_ptr);
                } front_char.is_some() {
                    if !front_pointers.is_empty() && front_pointers.len() < needed {
                        // not able to find free space, we advance to the next file
                    }
                }
                    if needed == 1 {
                        block_map.swap(front_ptr, start_back_ptr);
                        front_ptr += 1;
                    } else {
                        let mut front_ptrs: Vec<usize> = vec![front_ptr];
                    }
                }
            } else {
                front_ptr +                back_ptr -= 1;
            }
        }
    }

    block_map

}
*/

fn map_to_items(file_item: &FileItem) -> VecDeque<(Option<usize>, u8)> {
    let mut items: VecDeque<(Option<usize>, u8)> = VecDeque::new();
    for _ in 0..file_item.blocks {
        items.push_back((file_item.id, file_item.blocks));
    }
    items
}

fn remap_blocks(mut block_map: VecDeque<Option<usize>>) -> VecDeque<Option<usize>> {

    let mut front_ptr = 0usize;
    let mut back_ptr = block_map.len() - 1;

    while back_ptr > front_ptr {
        if !need_block_remap(&block_map) {
            break;
        } else {
            let front_char = block_map.get(front_ptr);
            let back_char = block_map.get(back_ptr);
            if front_char.is_some() && back_char.is_some() {
                if front_char.unwrap().is_some() {
                    front_ptr += 1;
                    continue;
                } 
                if back_char.unwrap().is_none() {
                    back_ptr -= 1;
                    continue;
                }
                block_map.swap(front_ptr, back_ptr);
                front_ptr += 1;
                back_ptr -= 1;
            }
        }
    }

    block_map

}

fn need_block_remap(blocks: &VecDeque<Option<usize>>) -> bool {
    let mut next_segment: bool = false;
    while !next_segment {
        for b in blocks.iter() {
            if !next_segment && b.is_none() {
                next_segment = true;
                continue;
            }
            if next_segment && b.is_some() {
                return true;
            }
        }
    }
    false
}

fn map_to_blocks(file_item: &FileItem) -> VecDeque<Option<usize>> {
    let mut blocks: VecDeque<Option<usize>> = VecDeque::new();
    for _ in 0..file_item.blocks {
        blocks.push_back(file_item.id.clone());
    }
    blocks
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

fn filesystem_checksum(blocks: VecDeque<Option<usize>>) -> Result<usize, String> {
    let mut check_sum = 0usize;
    for (i, x) in blocks.iter().enumerate() {
        if let Some(y) = x {
            check_sum += i * *y;
        }
    }
    Ok(check_sum)
}

// -------------------- PUZZLE PART 1 ------------------ //
fn puzzle_solve1(data: &Vec<String>) -> Result<usize, String> {
    let disk_map = DiskMap::init(data)?;
    let remapped = remap_blocks(disk_map.diskmap_to_blockmap());
    filesystem_checksum(remapped)
}

// -------------------- PUZZLE PART 2 ------------------ //
fn puzzle_solve2(data: &Vec<String>) -> Result<usize, String> {
    let disk_map = DiskMap::init(data)?;
    //let remapped = remap_items(disk_map.diskmap_to_itemsmap());
    //filesystem_checksum(remapped)
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
        let test_expected = 1928usize;

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
        let test_expected = 2858usize;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(puzzle_solve2(&d)?, test_expected);

        Ok(())
    }
}