
/* *************************************************************************
                            LIBRARIES AND DECLARATIONS
   ************************************************************************* */

use std::collections::{HashSet, VecDeque};

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
    disk_items: Vec<FileItem>
}

impl DiskMap {
    fn init(data: &Vec<String>) -> Result<DiskMap, String> {
        let mut disk_items: Vec<FileItem> = vec![];
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
        Ok(DiskMap { raw, disk_items })
    }

    fn diskmap_to_blockmap(&self) -> Vec<Option<usize>> {
        let mut block_map: Vec<Option<usize>> = vec![];
        for b in self.disk_items.iter() {
            let mut blocks: Vec<Option<usize>> = map_to_blocks(b);
            block_map.append(&mut blocks);
        }
        block_map
    }

    fn diskmap_to_itemsmap(&self) -> Vec<(Option<usize>, u8)> {
        let mut items_map: Vec<(Option<usize>, u8)> = vec![];
        for b in self.disk_items.iter() {
            let mut items: Vec<(Option<usize>, u8)> = map_to_items(b);
            items_map.append(&mut items);
        }
        items_map
    }

}

/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */

// Returns a pointer to the index on items_map containing needed free space or returns None
fn find_free_space(items_map: &Vec<Option<usize>>, needed_size: u8, before_idx: Option<usize>) -> Option<usize> {

    if let Some(idx) = before_idx {
        //println!("        *** Here 1");
        let mut free_space_ptr: Option<usize> = None; 
        let mut free_space_count = 0u8;
        for (free_space_idx, file_id) in items_map.iter().enumerate() {
            if (free_space_idx + needed_size as usize) >= idx {
                //println!("        *** Here 2");
                return None;
            }
            if file_id.is_some() {
                //println!("        *** Here 3");
                if free_space_count > 0 && free_space_ptr.is_some() {
                    if free_space_count >= needed_size && free_space_idx < idx {
                        //println!("        *** Here 0003");
                        return free_space_ptr
                    } else {
                        //println!("        *** Here 4");
                        // reset until next occurrence of free space
                        free_space_count = 0u8;
                        free_space_ptr = None;
                    }
                }
            } else {
                //println!("        *** Here 5");
                if free_space_ptr.is_none() {
                    //println!("        *** Here 6");
                    free_space_ptr = Some(free_space_idx);
                }
                free_space_count += 1; 
            }
        }
    }
    None
}

fn remap_items(items_map: &Vec<(Option<usize>, u8)>, mut files_list: VecDeque<(usize, usize, u8)>) -> Vec<Option<usize>> {
    let mut map_items: Vec<Option<usize>> = vec![];
    for n in items_map.into_iter() {
        map_items.push(n.0);
    }

    // TODO: Remove for debugging only
    //println!("\nOriginal blocks:");
    //let _ = &map_items.iter().for_each(|x|println!("    Orig: {:?}", x));
    //println!("\nRemaping Process:");

    while let Some(file_item) = files_list.pop_back() {

        // TODO Remove for debugging only
        //println!("    Remapping item: {:?}", &file_item);

        // make it easier to remember
        let file_index = file_item.0;
        let file_size = file_item.2;
        if let Some(free_space_idx) = find_free_space(&map_items, file_size, Some(file_index)) {

            // TODO Remove for debugging only
            //println!("    --- Found free space at index: {:?}", &free_space_idx);

            let mut free_ptr = free_space_idx;
            let mut file_ptr  = file_index;
            let mut count = 0u8;
            while count < file_size {
                map_items.swap(free_ptr, file_ptr);
                free_ptr += 1;
                file_ptr += 1;
                count += 1;
            }
        }
    }

    // TODO: Remove for debugging only
    //println!("\nRemapped blocks:");
    //let _ = &map_items.iter().for_each(|x|println!("    Remapped: {:?}", x));

    map_items
}

// Return only items that are files,  in an array of --------> index, fileid, blocks
// Will be used to keep track of files processed for remap.     \/     \/     \/
fn get_files(items_map: &Vec<(Option<usize>, u8)>) -> VecDeque<(usize, usize, u8)> {
    let mut files: VecDeque<(usize, usize, u8)> = VecDeque::new();
    let mut unique_ids: HashSet<usize> = HashSet::new();
    for (idx, item) in items_map.iter().enumerate() {
        if let Some(file_id) = item.0 {
            // We just need 1 instance of the file if file uses multiple blocks.
            // Note that items_map is a collections with each element representing a block
            if !unique_ids.contains(&file_id) {
                files.push_back((idx, file_id, item.1));
                unique_ids.insert(file_id);
            }
        }
    }
    files
}

fn map_to_items(file_item: &FileItem) -> Vec<(Option<usize>, u8)> {
    let mut items: Vec<(Option<usize>, u8)> = vec![];
    for _ in 0..file_item.blocks {
        items.push((file_item.id, file_item.blocks));
    }
    items
}

fn remap_blocks(mut block_map: Vec<Option<usize>>) -> Vec<Option<usize>> {

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

fn need_block_remap(blocks: &Vec<Option<usize>>) -> bool {
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

fn map_to_blocks(file_item: &FileItem) -> Vec<Option<usize>> {
    let mut blocks: Vec<Option<usize>> = vec![];
    for _ in 0..file_item.blocks {
        blocks.push(file_item.id.clone());
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

fn filesystem_checksum(blocks: Vec<Option<usize>>) -> Result<usize, String> {
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
    let items_map: Vec<(Option<usize>, u8)> = disk_map.diskmap_to_itemsmap();
    let files_list: VecDeque<(usize, usize, u8)> = get_files(&items_map);
    let remapped = remap_items(&items_map, files_list);
    filesystem_checksum(remapped)
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

    //println!("---------------");
    //println!("Solve Part 1:");
    //println!("---------------\n");
    //println!("  Part 1 Result: {:?}\n\n", puzzle_solve1(&data)?);

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