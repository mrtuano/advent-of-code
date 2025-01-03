
/* *************************************************************************
                          LIBRARIES
************************************************************************* */
use std::collections::{HashMap, HashSet, VecDeque};


/* *************************************************************************
                            TYPES
   ************************************************************************* */
type Point = (i32, i32);


/* *************************************************************************
                            TRAITS
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn vector(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0)
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Wall,
    Box,
    LBox,
    RBox,
    Robot,
    None
}

impl Type {
    fn draw(&self) -> char {
        match self {
            Type::Wall => '#',
            Type::Box => 'O',
            Type::LBox => '[',
            Type::RBox => ']',
            Type::Robot => '@',
            Type::None => '.'
        }
    }

    fn parse(c: &char) -> Type {
        match c {
            '#' => Type::Wall,
            '@' => Type::Robot,
            'O' => Type::Box,
            '[' => Type::LBox,
            ']' => Type::RBox,
            _ => Type::None
        }
    }
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

// ----------------------------------------------- //
// The Objects in the Warehouse
// Separately track the position of the robot.
// ----------------------------------------------- //
#[derive(Debug)]
struct Warehouse {
    objects: HashMap<Point, Type>,
    robot: Point
}

impl Warehouse {
    fn robot_attempt_move(&self, direction: &Direction) -> Option<Vec<(Point, Point)>> {

        let mut buffer: Vec<(Point, Point)> = vec![];
        let mut current: VecDeque<Point> = VecDeque::from(vec![self.robot]);

        let max: &Point = self.objects.keys().max()?;

        // Special parsing for the robot, see what's ahead of our robot
        if let Some((ahead, p)) = whats_ahead(&self.robot, direction, self) {
            match ahead {
                Type::None => {
                    buffer.push((self.robot, p));
                    return Some(buffer);
                },
                Type::Wall => {
                    return None;
                }
                _ => ()
            };
        };

        // Limit our loop so that we won't go on forever and ever
        for _i in 0..max.0 {
            while let Some(c) = current.pop_front() {
                if let Some((o, ahead)) = whats_ahead(&c, direction, self) {
                    if o == Type::LBox {
                        buffer.push((c, ahead));
                        current.push_back(ahead);
                        if *direction == Direction::Up || *direction == Direction::Down {
                            let box_point2 = (ahead.0 + 1, ahead.1);
                            if let Some(o2) = self.objects.get(&box_point2) {
                                if *o2 == Type::RBox {
                                    current.push_back(box_point2);
                                }
                            };
                        }
                    } else if o == Type::RBox {
                        buffer.push((c, ahead));
                        current.push_back(ahead);
                        if *direction == Direction::Up || *direction == Direction::Down {
                            let box_point2 = (ahead.0 - 1, ahead.1);
                            if let Some(o2) = self.objects.get(&box_point2) {
                                if *o2 == Type::LBox {
                                    current.push_back(box_point2);
                                }
                            };
                        }
                    } else if o == Type::None {
                        buffer.push((c, ahead));
                    } else if o == Type::Wall {
                        return None;
                    }
                };
            }
            return Some(buffer);
        }
        None
    }

    fn dump(&self) {
        let max: &Point = self.objects.keys().max().unwrap_or(&(0,0));
        for y in 0..=max.1 {
            let mut line_chars: Vec<char> = vec![];
            for x in 0..=max.0 {
                if let Some(o) = self.objects.get(&(x, y)) {
                    let s = o.draw();
                    line_chars.push(s);
                }
            }
            let line: String = line_chars.iter().collect();
            println!("{:?}", line);
        }
    }
}


/* *************************************************************************
                            FUNCTIONS
   ************************************************************************* */
fn whats_ahead(p: &Point, d: &Direction, w: &Warehouse) -> Option<(Type, Point)> {
    if let Some(n) = peek_ahead(p, d.vector()) {
        if let Some(o) = w.objects.get(&n) {
            return Some((*o, n));
        } else {
            return None;
        };
    } else {
        return None;
    };
}

fn peek_ahead(p: &Point, v: (i8, i8)) -> Option<Point> {
    Some(
        (
            p.clone().0.checked_add(v.0 as i32)?,
            p.clone().1.checked_add(v.1 as i32)?
        )
    )
}

fn read_data(data: &Vec<String>) -> (Warehouse, Vec<Direction>) {

    let mut robot: Point = (0, 0);
    let mut objects: HashMap<Point, Type> = HashMap::new();
    let mut directions: Vec<Direction> = vec![];

    // For part 2, supersize (2x wider) everything except the robot.
    let mut x = 0i32;
    let mut y = 0i32;

    for l in data.iter() {
        // Parse the warehouse map
        if l.contains('#') {
            for c in l.chars() {
                let t = Type::parse(&c);
                let p: Point = (x, y);
                if t == Type::Box {
                    objects.entry(p).or_insert(Type::LBox);
                } else {
                    objects.entry(p).or_insert(t);
                }

                x += 1;
                let p2 = (x, y);

                if t == Type::Robot {
                    robot = p;
                    objects.entry(p2).or_insert(Type::None);
                } else if t == Type::Box {
                    objects.entry(p2).or_insert(Type::RBox);
                } else {
                    objects.entry(p2).or_insert(t);
                }

                x += 1;
            }
            x = 0;
            y += 1;

        } else if l.contains(['<', 'v', '>', '^']) {
        // Parse the robot directions
            for c in l.chars() {
                let d = match c {
                    '<' => Direction::Left,
                    'v' => Direction::Down,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    _ => continue
                };
                directions.push(d);
            }

       }
    }

    (Warehouse {objects, robot}, directions)
}

pub fn solve_part2(data: &Vec<String>) -> Result<u64, String> {
    let (mut warehouse, directions) = read_data(data);

    // TODO: Remove, for debugging only.
    println!("\nBEFORE ALL MOVEMENTS:");
    warehouse.dump();

    for d in directions.iter() {
        let mut tracker: HashSet<Point> = HashSet::new();
        if let Some(new_positions) = warehouse.robot_attempt_move(d) {
            for (old_position, new_position) in new_positions.iter().rev() {
                // Keep track of previous already done old locations to prevent 
                // moving the objects twice.
                if tracker.insert(*old_position) {
                    if let Some(o) = warehouse.objects.remove(old_position) {
                        if o == Type::Robot {
                            warehouse.objects.insert(*old_position, Type::None);
                            warehouse.robot = *new_position;
                        } else if o == Type::LBox || o == Type::RBox {
                            warehouse.objects.insert(*old_position, Type::None);
                        }
                        warehouse.objects.insert(*new_position, o);
                    }
                }
            }
        }
    }
    
    // TODO: Remove, for debugging only.
    println!("\n\nAFTER ALL MOVEMENTS:");
    warehouse.dump();

    let sum_gps: u64 = warehouse.objects.iter()
        .filter(|(_p, o)| **o == Type::LBox)
        .map(|(p, _)| p.0 as u64 + 100*p.1 as u64)
        .sum();

    Ok(sum_gps)
}

/* *************************************************************************
                         TESTING
   ************************************************************************* */
#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::aoc_utils::*;

    #[test]
    fn test_solve_part1() -> Result<(), String> {
        
        // Update as needed
        let test_input = "test.data";
        //let test_input = "test.data.small2";
        let test_expected = 9021u64;

        // Read in test data
        let d= PuzzleInput::init(Some(&["this".to_string(), test_input.to_string()]))?
            .vectorized()?;

        // Test our solution
        assert_eq!(solve_part2(&d)?, test_expected);

        Ok(())
    }
}