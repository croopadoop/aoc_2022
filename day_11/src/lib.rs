mod helpers;
const INPUT: &str = "./input.txt";

#[derive(Debug, Clone, Default)]
struct Monkey {
    items_held: Vec<u64>,
    inspect_item: Operation,
    test: u64,
    dest: (usize, usize),
    items_inspected: u64,
}

#[derive(Debug, Copy, Clone, Default)]
enum Operation {
    #[default]
    AddSelf,
    MulSelf,
    Mul(u64),
    Add(u64),
}

impl Operation {
    fn run(&self, x: u64) -> u64 {
        match self {
            Self::AddSelf => x + x,
            Self::Add(y) => x + y,
            Self::MulSelf => x * x,
            Self::Mul(y) => x * y,
        }
    }
}

fn parse_input_to_monkies(path: &str) -> Vec<Monkey> {
    let mut monkies: Vec<Monkey> = Vec::new();
    let mut monkey = Monkey::default();
    let input_lines = helpers::read_lines(INPUT).unwrap();
    for line in input_lines {
        match line {
            Ok(l) => {
                let words = l.trim().split(" ").collect::<Vec<&str>>();
                match words[0] {
                    "Monkey" => {
                        monkey.items_inspected = 0;
                    }
                    "Starting" => {
                        let items = words.clone().drain(2..).collect::<Vec<&str>>();
                        for item in items {
                            monkey
                                .items_held
                                .push(item.replace(",", "").parse::<u64>().unwrap());
                        }
                    }
                    "Operation:" => {
                        monkey.inspect_item = if words[4] == "+" {
                            if words[5] == "old" {
                                Operation::AddSelf
                            } else {
                                let add_to = words[5].parse::<u64>().unwrap();
                                Operation::Add(add_to)
                            }
                        } else {
                            if words[5] == "old" {
                                Operation::MulSelf
                            } else {
                                let mul_by = words[5].parse::<u64>().unwrap();
                                Operation::Mul(mul_by)
                            }
                        }
                    }
                    "Test:" => {
                        monkey.test = words[3].parse::<u64>().unwrap();
                    }
                    "If" => {
                        if words[1] == "true:" {
                            monkey.dest.0 = words[5].parse::<usize>().unwrap();
                        } else {
                            monkey.dest.1 = words[5].parse::<usize>().unwrap();
                            monkies.push(monkey);
                            monkey = Monkey::default();
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => {
                panic!("Oh no!");
            }
        }
    }

    monkies
}

fn process_round(monkies: &mut Vec<Monkey>) {
    // Cycle through the monkies
    for i in 0..monkies.len() {
        // For all items held by the monkey
        while let Some(item) = monkies[i].items_held.pop() {
            // Take an item and run the inspect, then divide by three to get the worry level
            let worry_level = monkies[i].inspect_item.run(item) / 3;

            // Test the worry level to determine where the monkey will throw the item
            let dest = if worry_level % monkies[i].test == 0 {
                monkies[i].dest.0
            } else {
                monkies[i].dest.1
            };

            // Throw the item to the monkey recipient based on the test
            monkies[dest].items_held.push(worry_level);

            // Increment the number of items this monkey has inspected
            monkies[i].items_inspected += 1;
        }
    }
}

fn process_round_without_worry(monkies: &mut Vec<Monkey>) {
    // Cycle through the monkies to get the product of their divisors
    let mut modulus: u64 = 1;
    for i in 0..monkies.len() {
        modulus *= monkies[i].test;
    }

    for i in 0..monkies.len() {
        // For all items held by the monkey
        while let Some(item) = monkies[i].items_held.pop() {
            // Take an item and run the inspect
            let worry_level = monkies[i].inspect_item.run(item);

            // Test the worry level to determine where the monkey will throw the item
            let dest = if worry_level % monkies[i].test == 0 {
                monkies[i].dest.0
            } else {
                monkies[i].dest.1
            };

            // Throw the item to the monkey recipient based on the test
            monkies[dest].items_held.push(worry_level % modulus);

            // Increment the number of items this monkey has inspected
            monkies[i].items_inspected += 1;
        }
    }
}

pub fn process_part1() -> u64 {
    // Retrieve monkies from the input lines
    let mut monkies = parse_input_to_monkies(INPUT);

    // Set the number of rounds
    let number_of_rounds = 20;

    for _ in 0..number_of_rounds {
        process_round(&mut monkies);
    }

    // Determine the two most active monkies
    monkies.sort_by(|monkey1, monkey2| monkey2.items_inspected.cmp(&monkey1.items_inspected));
    let most_active_monkey = monkies[0].items_inspected;
    let second_most_active_monkey = monkies[1].items_inspected;

    // Return the product of the two most active monkies
    most_active_monkey * second_most_active_monkey
}

pub fn process_part2() -> u64 {
        // Retrieve monkies from the input lines
        let mut monkies = parse_input_to_monkies(INPUT);

        // Set the number of rounds
        let number_of_rounds = 10000;
    
        for _ in 0..number_of_rounds {
            process_round_without_worry(&mut monkies);
        }
    
        // Determine the two most active monkies
        monkies.sort_by(|monkey1, monkey2| monkey2.items_inspected.cmp(&monkey1.items_inspected));
        let most_active_monkey = monkies[0].items_inspected;
        let second_most_active_monkey = monkies[1].items_inspected;

        for monkey in monkies {
            println!("{:?}", monkey);
        }
    
        // Return the product of the two most active monkies
        most_active_monkey * second_most_active_monkey
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 10605);
    }

    #[test]
    #[ignore]
    fn process_part2_works() {
        // let result = process_part2();
        // assert_eq!(result, 36);
    }
}
