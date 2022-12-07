// For all backpacks
// 1. Get backpack.
// 2. Split compartments.
// 3. For all items in compartment 1, search compartment 2 for a match.
// 4. When a match if found, get the priority of the match
// 5. Print running total.

pub fn process_part1(input: &str) -> String {
    let priority = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let backpacks = input.split("\r\n").collect::<Vec<_>>();
    let mut total_priority_of_duplicates = 0;

    for backpack in backpacks {
        println!("{}", backpack.len());
        let compartment_split_idx = backpack.len() / 2;
        println!("split_idx: {}", compartment_split_idx);
        let compartments = backpack.split_at(compartment_split_idx);
        println!("{:?}", compartments);
        println!("compartment_1 len: {}", compartments.0.len());
        println!("compartment_2 len: {}", compartments.1.len());
        let mut duplicate_item: char = '_';
        for compartment_1_item in compartments.0.chars() {
            for compartment_2_item in compartments.1.chars() {
                if compartment_1_item == compartment_2_item {
                    duplicate_item = compartment_1_item;
                    break;
                }
            }
        }

        let priority_value = priority.iter().position(|item| item == &duplicate_item).unwrap() + 1;
        total_priority_of_duplicates += priority_value;
    }

    total_priority_of_duplicates.to_string()
}

// Take three vectors as a group.
// Search vector 2 for a common char
// Search vector 3 for a common char

pub fn process_part2(input: &str) -> String {
    let priority = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut backpacks = input.split("\r\n").collect::<Vec<_>>();
    let mut idx = 0;
    let mut total_score = 0;

    while idx != backpacks.len() {
        let backpack_1 = backpacks[0 + idx];
        let backpack_2 = backpacks[1 + idx];
        let backpack_3 = backpacks[2 + idx];

        println!("{} {} {}", backpack_1, backpack_2, backpack_3);
        let mut team_badge: char = '_';
        for backpack_1_item in backpack_1.chars() {
            for backpack_2_item in backpack_2.chars() {
                for backpack_3_item in backpack_3.chars() {
                    if backpack_1_item == backpack_2_item && backpack_1_item == backpack_3_item {
                        team_badge = backpack_1_item;
                    }
                }
            }
        }
        let team_priority = priority.iter().position(|item| item == &team_badge).unwrap() + 1;
        total_score += team_priority;
        idx += 3;
    }

    total_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\r\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\r\nPmmdzqPrVvPwwTWBwg\r\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\r\nttgJtRGJQctTZtZT\r\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn process_part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
