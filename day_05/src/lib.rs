mod helpers;

#[derive(Debug)]
pub struct Move {
    count: usize,
    from_idx: usize,
    to_idx: usize,
}

pub fn process_part1() -> String {
    let mut stacks = get_crates();
    let instructions = get_instructions();

    for instruction in instructions {
        for _ in 1..=instruction.count {
            let crate_to_move = stacks[instruction.from_idx].pop().unwrap();
            stacks[instruction.to_idx].push(crate_to_move);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        result.push(stack.last().unwrap().to_owned());
    }

    result
}

pub fn process_part2() -> String {
    let mut stacks = get_crates();
    let instructions = get_instructions();

    for instruction in instructions {
        let removal_rng = stacks[instruction.from_idx].len() - instruction.count;
        let crate_to_move: Vec<_> = stacks[instruction.from_idx].drain(removal_rng..).collect();
        
        for c in crate_to_move
        {
            stacks[instruction.to_idx].push(c);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        result.push(stack.last().unwrap().to_owned());
    }

    result
}

pub fn get_crates() -> Vec<Vec<char>> {
    let stack_rows = helpers::read_lines("./input_start.txt").unwrap();
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();

    for stack_row in stack_rows {
        match stack_row {
            Ok(row) => {
                println!("Row to review: {}", row);
                let mut push_to_idx = 0;
                let mut next_idx = 1;
                for (i, c) in row.chars().enumerate() {
                    if i == next_idx {
                        if push_to_idx >= crate_stacks.len() {
                            crate_stacks.push(Vec::new());
                        }
                        next_idx += 4;
                        if c != ' ' {
                            crate_stacks[push_to_idx].push(c);
                        }
                        push_to_idx += 1;
                    }
                }
            }
            Err(_) => {
                eprintln!("failed to read line");
            }
        }
    }

    // Reverse Stacks
    let mut rev_stacks: Vec<Vec<char>> = Vec::new();
    for mut stack in crate_stacks {
        stack.reverse();
        rev_stacks.push(stack);
    }

    rev_stacks
}

pub fn get_instructions() -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let instructions = helpers::read_lines("./input.txt").unwrap();

    for instruction in instructions {
        match instruction {
            Ok(direction) => {
                let set = direction.split(" ").collect::<Vec<&str>>();
                moves.push(Move {
                    count: set[1].parse::<usize>().unwrap(),
                    from_idx: set[3].parse::<usize>().unwrap() - 1,
                    to_idx: set[5].parse::<usize>().unwrap() - 1,
                })
            }
            Err(_) => {
                eprintln!("error reading line");
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2();
        assert_eq!(result, "MCD");
    }
}
