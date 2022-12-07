pub fn process_part1(input: &str) -> String {
    println!("{}", input);
    let result = input
        .lines()
        .map(|x| {
            x.split(",")
                .map(|y| {
                    y.split("-")
                        .map(|idx| idx.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_overlap = 0;

    for pair in result {
        let assignment_1: Vec<u32> = (pair[0][0]..=pair[0][1]).collect();
        let assignment_2: Vec<u32> = (pair[1][0]..=pair[1][1]).collect();
        let len_1 = assignment_1.len();
        let len_2 = assignment_2.len();
        let mut count_overlap = 0;

        if len_1 >= len_2 {
            for seat in assignment_2 {
                if assignment_1.contains(&seat) {
                    count_overlap += 1;
                }
            }

            if count_overlap == len_2 {
                total_overlap += 1;
            }
        } else {
            for seat in assignment_1 {
                if assignment_2.contains(&seat) {
                    count_overlap += 1;
                }
            }

            if count_overlap == len_1 {
                total_overlap += 1;
            }
        }
    }

    total_overlap.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|x| {
            x.split(",")
                .map(|y| {
                    y.split("-")
                        .map(|idx| idx.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_overlap = 0;

    for pair in result {
        let assignment_1: Vec<u32> = (pair[0][0]..=pair[0][1]).collect();
        let assignment_2: Vec<u32> = (pair[1][0]..=pair[1][1]).collect();

        for seat in assignment_1 {
            if assignment_2.contains(&seat) {
                total_overlap += 1;
                break;
            }
        }
    }

    total_overlap.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8\r\n2-3,4-5\r\n5-7,7-9\r\n2-8,3-7\r\n6-6,4-6\r\n2-6,4-8";

    #[test]
    fn process_part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
