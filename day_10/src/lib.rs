mod helpers;
// const INPUT_START: &str = "./test_start.txt";
const INPUT: &str = "./input.txt";

pub fn process_part1() -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut next_breakpoint = 20;
    let mut result: i32 = 0;
    let mut screen = String::new();

    let instructions = helpers::read_lines(INPUT).unwrap();
    for instruction in instructions {
        let mut add_to_x = 0;
        match instruction {
            Ok(command) => {
                let mut no_of_cycles = 0;
                if command.starts_with("addx") {
                    let cmd = command.split_whitespace().collect::<Vec<&str>>();
                    add_to_x = cmd[1].parse::<i32>().unwrap();
                    no_of_cycles += 2;
                } else {
                    no_of_cycles += 1;
                }

                for _ in 0..no_of_cycles {
                    cycle += 1;
                    if cycle == next_breakpoint {
                        let output = next_breakpoint * x;
                        result += output;
                        println!("Breakpoint: {}, Output: {}", next_breakpoint, output);
                        next_breakpoint += 40;
                    }
                }

                x += add_to_x;
            }
            Err(_) => {
                eprintln!("Something went wrong");
            }
        }
    }

    result
}

pub fn process_part2() -> String {
    let mut sprite_horizontal_position: i32 = 1;
    let mut cycle_number_completed = 0;
    let mut next_breakpoint = 40;
    let mut screen: Vec<String> = Vec::new();
    let mut final_image = String::new();

    let instructions = helpers::read_lines(INPUT).unwrap();
    for instruction in instructions {
        if screen.len() == 0 {
            screen.push(String::new());
        }
        let mut horizontal_positional_change: i32 = 0;
        match instruction {
            Ok(command) => {
                let mut number_of_cycles = 0;
                if command.starts_with("addx") {
                    let cmd = command.split_whitespace().collect::<Vec<&str>>();
                    horizontal_positional_change = cmd[1].parse::<i32>().unwrap();
                    number_of_cycles += 2;
                } else {
                    number_of_cycles += 1;
                }

                for _ in 0..number_of_cycles {
                    if cycle_number_completed == next_breakpoint {
                        // Start a new line
                        screen.push(String::new());

                        // Set next breakpoint
                        next_breakpoint += 40;
                    }

                    // Retrieve line to draw
                    let line_number_being_drawn = screen.len() - 1;
                    let position_to_draw_next = screen[line_number_being_drawn].len() as i32;

                    if (position_to_draw_next == sprite_horizontal_position - 1)
                        || (position_to_draw_next == sprite_horizontal_position)
                        || (position_to_draw_next == sprite_horizontal_position + 1)
                    {
                        screen[line_number_being_drawn].push('#');
                    } else {
                        screen[line_number_being_drawn].push('.');
                    }

                    cycle_number_completed += 1
                }

                sprite_horizontal_position += horizontal_positional_change;
            }
            Err(_) => {
                eprintln!("Something went wrong");
            }
        }
    }
    for line in &screen {
        let line = line.to_owned() + "\r\n";
        final_image.push_str(line.as_str());
    }

    final_image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 13140);
    }

    #[test]
    #[ignore]
    fn process_part2_works() {
        // let result = process_part2();
        // assert_eq!(result, 36);
    }
}
