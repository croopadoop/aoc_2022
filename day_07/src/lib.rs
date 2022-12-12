use std::collections::HashMap;
mod helpers;

pub fn process_part1() -> i32 {
    let input_lines = helpers::read_lines("./test.txt").unwrap();
    let mut path = String::new();
    let mut paths: Vec<String> = Vec::new();
    let mut path_history: Vec<String> = Vec::new();
    let mut raw_path_sizes: HashMap<String, i32> = HashMap::new();
    let mut dir_size = 0;
    let mut result = 0;

    for line in input_lines {
        match line {
            Ok(statement) => {
                let statement_parts = statement.split(" ").collect::<Vec<_>>();
                if statement_parts[0] == "$" {
                    // Map commands
                    if statement_parts[1] == "cd" {
                        //println!("{}", path);
                        //Map directory change
                        match statement_parts[2] {
                            ".." => {
                                path_history.pop();
                                path = path_history.last().unwrap().to_owned();
                            }
                            "/" => {
                                path_history.clear();
                                path.clear();
                                path.push_str("/");
                                path_history.push(path.clone());
                            }
                            "" => {}
                            _ => {
                                let new_dir = statement_parts[2].to_owned() + "/";
                                path.push_str(new_dir.as_str());
                                paths.push(path.clone());
                                path_history.push(path.clone());
                            }
                        }
                    } else {
                        // ls
                        dir_size = 0;
                    }
                } else if statement_parts[0] == "dir" {

                } else {
                    // File
                    dir_size += statement_parts[0].parse::<i32>().unwrap();
                    raw_path_sizes.insert(path.clone(), dir_size);
                }
            }
            Err(_) => {
                eprintln!("Error reading line");
            }
        }
    }

    for path in paths {
        let mut total_path_size = 0;
        for k in raw_path_sizes.keys() {
            if k.contains(&path) {
                let v = raw_path_sizes.get(k).unwrap();
                total_path_size += v;
            }
        }

        if total_path_size <= 100000 {
            result += total_path_size;
        }
    }

    result
}

pub fn process_part2() -> i32 {
    let input_lines = helpers::read_lines("./input.txt").unwrap();
    let mut path = String::new();
    let mut paths: Vec<String> = Vec::new();
    let mut path_history: Vec<String> = Vec::new();
    let mut raw_path_sizes: HashMap<String, i32> = HashMap::new();
    let mut dir_size = 0;

    for line in input_lines {
        match line {
            Ok(statement) => {
                let statement_parts = statement.split(" ").collect::<Vec<_>>();
                if statement_parts[0] == "$" {
                    // Map commands
                    if statement_parts[1] == "cd" {
                        //Map directory change
                        match statement_parts[2] {
                            ".." => {
                                path_history.pop();
                                path = path_history.last().unwrap().to_owned();
                            }
                            "/" => {
                                path_history.clear();
                                path.clear();
                                path.push_str("/");
                                paths.push(path.clone());
                                path_history.push(path.clone());
                            }
                            "" => {}
                            _ => {
                                let new_dir = statement_parts[2].to_owned() + "/";
                                path.push_str(new_dir.as_str());
                                paths.push(path.clone());
                                path_history.push(path.clone());
                            }
                        }
                    } else {
                        // ls
                        dir_size = 0;
                    }
                } else if statement_parts[0] == "dir" {

                } else {
                    // File
                    dir_size += statement_parts[0].parse::<i32>().unwrap();
                    raw_path_sizes.insert(path.clone(), dir_size);
                }
            }
            Err(_) => {
                eprintln!("Error reading line");
            }
        }
    }

    let mut result: i32 = 0;
    let mut path_sizes: HashMap<String, i32> = HashMap::new();

    for path in paths {
        let mut total_path_size = 0;
        for k in raw_path_sizes.keys() {
            if k.contains(&path) {
                let v = raw_path_sizes.get(k).unwrap();
                total_path_size += v;
            }
        }

        path_sizes.insert(path, total_path_size);
    }

    let total_used = path_sizes.get("/").unwrap();
    let unused_space = 70000000 - total_used;
    let space_to_free = 30000000- unused_space;

    println!("total used: {}", total_used);
    println!("unused space: {}", unused_space);
    println!("space to free: {}", space_to_free);

    for (path, size) in path_sizes {
        if size >= space_to_free && size <= result {
            println!("path to delete: {}", path);
            result = size;
        } else if size >= space_to_free && result == 0 {
            println!("path to delete: {}", path);
            result = size;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 95437);
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2();
        assert_eq!(result, 24933642);
    }
}
