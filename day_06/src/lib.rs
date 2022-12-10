mod helpers;

pub fn process_part1() -> usize {
    let input = helpers::read_string_from_file("./input.txt").unwrap();
    let drain_stop = input.len() - 4;
    let mut drain_to_idx = 3;
    let mut res = 0;

    for i in 0..=drain_stop {
        let mut input_clone = input.clone();
        let input_drain = input_clone.drain(i..=drain_to_idx);
        let seg_str = input_drain.as_str();
        let mut is_unique = true;

        for c in seg_str.chars() {
            let seg_copy = seg_str.clone();
            let seg_matches = seg_copy.matches(c);

            if seg_matches.count() != 1 {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            res = drain_to_idx + 1;
            break;
        }
        drain_to_idx += 1;
    }

    res
}

pub fn process_part2() -> usize {
    let input = helpers::read_string_from_file("./input.txt").unwrap();
    let drain_stop = input.len() - 14;
    let mut drain_to_idx = 13;
    let mut res = 0;

    for i in 0..=drain_stop {
        let mut input_clone = input.clone();
        let input_drain = input_clone.drain(i..=drain_to_idx);
        let seg_str = input_drain.as_str();
        let mut is_unique = true;

        for c in seg_str.chars() {
            let seg_copy = seg_str.clone();
            let seg_matches = seg_copy.matches(c);

            if seg_matches.count() != 1 {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            res = drain_to_idx + 1;
            break;
        }
        drain_to_idx += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 7);
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2();
        assert_eq!(result, 19);
    }
}
