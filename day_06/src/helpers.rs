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
