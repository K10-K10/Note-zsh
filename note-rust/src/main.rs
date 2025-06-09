#[allow(dead_code)]
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::{fs, mem};

struct Item {
    num: u32,
    title: String,
    body: String,
    check: bool,
}

fn list(path: File) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(path);

    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line)
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../note.txt")?;
    list(file)
}
