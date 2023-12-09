use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change input file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut tot = 0;

    for l in reader.lines() {
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
        let occurrencies = nums.into_iter()
        .filter(|&n|
            winning.contains(n)
        ).collect::<Vec<_>>()
        .len() as u32;

        if occurrencies != 0 {
            tot += usize::pow(2, occurrencies - 1);
        }
    }
    println!("{}", tot);


    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let s = String::from(" 1");
        assert_eq!(1, s.trim().parse().unwrap());
    }
}