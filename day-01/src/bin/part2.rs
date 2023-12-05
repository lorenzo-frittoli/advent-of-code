use std::fs::File;
use std::io::{prelude::*, BufReader};


fn parse(s: String) -> String {
    return s
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}

fn bugo(line_string: String) -> (u16, u16) {
    let arr: Vec<u16> = parse(line_string)
        .chars()
        .map(|c| (c as u16) - 0x30)
        .filter(|c| *c <= 9)
        .collect();
    (arr[0], arr[arr.len()-1])
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut tot = 0;

    for l in reader.lines() {
        let line_string = l.unwrap();
        let (a, b) = bugo(line_string);
        tot += a*10 + b;
    }

    println!("{}", tot);
}