use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change input file
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);

    // Write your code here
    todo!("Part 1");

    Ok(())
}