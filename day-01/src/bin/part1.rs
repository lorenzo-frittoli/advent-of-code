use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::iter::zip;

const BUGO: &'static str = "
import numpy as np
from matplotlib import pyplot as plt
";

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut tot = 0;

    for l in reader.lines() {
        let line_string = l?;
        let (a,b) = bugo(line_string);
        tot = tot + a*10 + b;
    }

    println!("{}", tot);

    Ok(())
}

fn bugo(line_string: String) -> (u16, u16) {
    let arr: Vec<u16> = line_string
        .chars()
        .map(|c| (c as u16) - 0x30)
        .filter(|c| *c <= 9)
        .collect();
    (arr[0], arr[arr.len()-1])
}