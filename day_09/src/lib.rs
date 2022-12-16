mod helpers;
const INPUT_START: &str = "./test_start.txt";
const INPUT: &str = "./input.txt";

pub fn process_part1() -> i32 {
    // Track Tail
    let matrix = helpers::parse_to_char_array(INPUT_START);
    let input_directions = helpers::read_lines(INPUT).unwrap();
    let mut places_t_has_been: Vec<(isize, isize)> = Vec::new();

    // Initialize H and T
    let mut h_arr_no: isize = 0;
    let mut h_arr_idx: isize = 0;
    let mut t_arr_no: isize = 0;
    let mut t_arr_idx: isize = 0;

    for line in matrix.clone() {
        let s_idx = line.iter().position(|&v| v == 'H');
        match s_idx {
            Some(idx) => {
                h_arr_idx = idx as isize;
                t_arr_idx = idx as isize;
            }
            None => {
                h_arr_no += 1;
                t_arr_no += 1;
            }
        }
    }

    let mut h: (isize, isize) = (h_arr_idx, h_arr_no);
    let mut t: (isize, isize) = (t_arr_idx, t_arr_no);

    for instruction in input_directions {
        match instruction {
            Ok(line) => {
                let direction = line.split_whitespace().collect::<Vec<_>>();
                let way = direction[0];
                let distance = direction[1].parse::<i32>().unwrap();

                for _ in 0..distance {
                    // Model H
                    if way == "L" {
                        h.0 -= 1;
                    } else if way == "U" {
                        h.1 -= 1;
                    } else if way == "R" {
                        h.0 += 1;
                    } else {
                        // "D"
                        h.1 += 1;
                    }

                    // Model T
                    let pos_h_from_t_x = h.0 as i32 - t.0 as i32;
                    let pos_h_from_t_y = h.1 as i32 - t.1 as i32;

                    if pos_h_from_t_y == 0 && pos_h_from_t_x == 2 {
                        // Move 'T' right one
                        t.0 += 1;
                    } else if pos_h_from_t_y == 0 && pos_h_from_t_x == -2 {
                        // Move 'T' left one
                        t.0 -= 1;
                    } else if pos_h_from_t_y == 2 && pos_h_from_t_x == 0 {
                        // Move 'T' down one
                        t.1 += 1;
                    } else if pos_h_from_t_y == -2 && pos_h_from_t_x == 0 {
                        // Move 'T' up one
                        t.1 -= 1;
                    } else if (pos_h_from_t_y == -2 && pos_h_from_t_x == 1)
                        || (pos_h_from_t_y == -1 && pos_h_from_t_x == 2)
                    {
                        // Move 'T' up one right one
                        t.1 -= 1;
                        t.0 += 1;
                    } else if (pos_h_from_t_y == -2 && pos_h_from_t_x == -1)
                        || (pos_h_from_t_y == -1 && pos_h_from_t_x == -2)
                    {
                        // Move 'T' up one left one
                        t.1 -= 1;
                        t.0 -= 1;
                    } else if (pos_h_from_t_y == 2 && pos_h_from_t_x == 1)
                        || (pos_h_from_t_y == 1 && pos_h_from_t_x == 2)
                    {
                        // Move 'T' down one right one
                        t.1 += 1;
                        t.0 += 1;
                    } else if (pos_h_from_t_y == 2 && pos_h_from_t_x == -1)
                        || (pos_h_from_t_y == 1 && pos_h_from_t_x == -2)
                    {
                        t.1 += 1;
                        t.0 -= 1;
                    } else {
                    }

                    // Check if T has been in its position before

                    let coord = places_t_has_been.clone().iter().position(|&c| c == t);
                    match coord {
                        Some(c) => {}
                        None => {
                            places_t_has_been.push(t);
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("File is empty");
            }
        }
    }

    places_t_has_been.len() as i32
}

pub fn process_part2() -> i32 {
    // Track Tail
    let input_directions = helpers::read_lines(INPUT).unwrap();
    let mut rope: Vec<(isize, isize)> = Vec::new();
    let mut places_t_has_been: Vec<(isize, isize)> = Vec::new();

    // Initialize H and T
    let mut s_y: isize = 0;
    let mut s_x: isize = 0;

    for _ in 0..=9 {
        rope.push((s_x, s_y));
    }

    for instruction in input_directions {
        match instruction {
            Ok(line) => {
                let direction = line.split_whitespace().collect::<Vec<_>>();
                let way = direction[0];
                let distance = direction[1].parse::<i32>().unwrap();

                for i in 0..distance {
                    // Model H
                    if way == "L" {
                        rope[0].0 -= 1;
                    } else if way == "U" {
                        rope[0].1 -= 1;
                    } else if way == "R" {
                        rope[0].0 += 1;
                    } else {
                        // "D"
                        rope[0].1 += 1;
                    }

                    for i in 1..10 {
                        rope[i] = should_move(rope[i], rope[i - 1]);
                    }

                    // Check if T has been in its position before
                    let t = rope.last().unwrap().to_owned();

                    let coord = places_t_has_been.clone().iter().position(|&c| c == t);
                    match coord {
                        Some(c) => {}
                        None => {
                            places_t_has_been.push(t);
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("File is empty");
            }
        }
    }
    places_t_has_been.len() as i32
}

pub fn should_move(mut this: (isize, isize), that: (isize, isize)) -> (isize, isize) {
    // Model T
    let pos_h_from_t_x = that.0 as i32 - this.0 as i32;
    let pos_h_from_t_y = that.1 as i32 - this.1 as i32;

    if pos_h_from_t_y == 0 && pos_h_from_t_x == 2 {
        // Move 'T' right one
        this.0 += 1;
    } else if pos_h_from_t_y == 0 && pos_h_from_t_x == -2 {
        // Move 'T' left one
        this.0 -= 1;
    } else if pos_h_from_t_y == 2 && pos_h_from_t_x == 0 {
        // Move 'T' down one
        this.1 += 1;
    } else if pos_h_from_t_y == -2 && pos_h_from_t_x == 0 {
        // Move 'T' up one
        this.1 -= 1;
    } else if (pos_h_from_t_y == -2 && pos_h_from_t_x == 1)
        || (pos_h_from_t_y == -1 && pos_h_from_t_x == 2)
        || (pos_h_from_t_y == -2 && pos_h_from_t_x == 2)
    {
        // Move 'T' up one right one
        this.1 -= 1;
        this.0 += 1;
    } else if (pos_h_from_t_y == -2 && pos_h_from_t_x == -1)
        || (pos_h_from_t_y == -1 && pos_h_from_t_x == -2)
        || (pos_h_from_t_y == -2 && pos_h_from_t_x == -2)
    {
        // Move 'T' up one left one
        this.1 -= 1;
        this.0 -= 1;
    } else if (pos_h_from_t_y == 2 && pos_h_from_t_x == 1)
        || (pos_h_from_t_y == 1 && pos_h_from_t_x == 2)
        || (pos_h_from_t_y == 2 && pos_h_from_t_x == 2)
    {
        // Move 'T' down one right one
        this.1 += 1;
        this.0 += 1;
    } else if (pos_h_from_t_y == 2 && pos_h_from_t_x == -1)
        || (pos_h_from_t_y == 1 && pos_h_from_t_x == -2)
        || (pos_h_from_t_y == 2 && pos_h_from_t_x == -2)
    {
        // Move 'T' down one left one
        this.1 += 1;
        this.0 -= 1;
    } else {
        // Do nothing
    }

    this
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 13);
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2();
        assert_eq!(result, 36);
    }
}
