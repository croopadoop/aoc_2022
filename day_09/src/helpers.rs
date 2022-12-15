use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::io::prelude::*;

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_string_from_file<P>(path: P) -> io::Result<String> where P: AsRef<Path>
{
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn parse_to_digit_array<P>(path: P) -> Vec<Vec<u32>> where P: AsRef<Path>
{
    let input_lines = read_lines(path).unwrap();
    let mut u32_lines:Vec<Vec<u32>> = Vec::new();

    for line in input_lines {
        let mut u32_line: Vec<u32> = Vec::new();
        match line {
            Ok(row) => {
                for t in row.chars() {
                    u32_line.push(t.to_digit(10).unwrap());
                }

                u32_lines.push(u32_line);
            }
            Err(_) => {
                eprintln!("Error reading line.");
            }
        }
    }

    u32_lines
}
