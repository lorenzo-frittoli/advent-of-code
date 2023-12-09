use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change input file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let filelen = BufReader::new(reader
        .get_ref())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .len();

    let mut numbers = vec![1_usize; filelen];
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for (i,l) in reader.lines().enumerate() {
        let parsed: Vec<Vec<usize>> = l?
            .split(": ")
            .last()
            .unwrap()
            .split(" | ")
            .map(|s| s
                .split_whitespace()
                .map(|n| n.parse::<usize>()
                .unwrap())
                .collect())
            .collect();

        let winning = &parsed[0];
        let nums = &parsed[1];
        let val = value(nums, winning);

        dbg!(i, numbers[i], val);
        if val == 0 {continue;}
        for j in (i+1)..((i+val+1).clamp(0, filelen)) {
            numbers[j] += numbers[i];
        }
    }

    let tot: usize = numbers.iter().sum();
    println!("{}", tot);


    Ok(())
}

fn value(nums: &Vec<usize>, winning: &Vec<usize>) -> usize {
    return nums
        .into_iter()
        .filter(|&n|
            winning.contains(n)
        ).collect::<Vec<_>>()
        .len();
}