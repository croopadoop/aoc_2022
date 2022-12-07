use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let elves: HashMap<i32, i32> = HashMap::new();

    let mut max_calories = 0;
    for elf in &elves {
        if elf.1 > &max_calories {
            max_calories = elf.1.to_owned();
        }
    }

    max_calories.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = 
    input
    .split("\n\n")
    .map(|elf_load| {
        elf_load
        .lines()
        .map(|item| item.parse::<u32>().unwrap())
        .sum::<u32>()
    })
    .collect::<Vec<u32>>();

    println!("{:?}", result);
    "not done".to_string()
}

// pub fn collect_elves(raw_data: &str) -> HashMap<i32, i32> {
//     // let split_input = raw_data.lines();
//     // let mut elves = HashMap::new();
//     // let mut elf_num = 1;

//     // for word in split_input {
//     //     if word.is_empty() {
//     //         elf_num += 1;
//     //         let new_elf = elf_num;
//     //         elves.insert(new_elf, 0);
//     //     }
//     //     else {
//     //         let calorie_count: i32 = word.parse().unwrap_or(0);
//     //         let total_calorie_count = elves.entry(elf_num).or_insert(0);
//     //         *total_calorie_count += calorie_count;
//     //     }
//     // }

//     // println!("elves map: {:?}", elves);
//     // elves


// }