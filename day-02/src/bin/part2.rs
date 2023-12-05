use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut tot = 0;

    for l in reader.lines() {
        let res = parse_full(&l?)?;
        tot += res;
    }
    println!("{}", tot);

    Ok(())
}

fn parse_full(s: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = s.split(": ").collect();
    // let i = parts[0][5..].parse()?;
    // let i = parts[0][5..].parse()?;
    let mut rgb: (usize, usize, usize) = (0,0,0);

    for round in parts[1].split("; ") {
        for merdolo in round.split(", ") {
            let stoca: Vec<&str> = merdolo.split(" ").collect();
            let num: usize = stoca[0].parse()?;
            match stoca[1] {
                "red" => rgb.0 = num.max(rgb.0),
                "green" => rgb.1 = num.max(rgb.1),
                "blue" => rgb.2 = num.max(rgb.2),
                _ => todo!(),
            }
        }
    }

    Ok(rgb.0 * rgb.1 * rgb.2)
}