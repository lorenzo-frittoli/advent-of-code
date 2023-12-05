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
    let i = parts[0][5..].parse()?;
    
    for round in parts[1].split("; ") {
        for merdolo in round.split(", ") {
            let stoca: Vec<&str> = merdolo.split(" ").collect();
            let num: usize = stoca[0].parse()?;
            match stoca[1] {
                "red" => if num > 12 {return Ok(0);},
                "green" => if num > 13 {return Ok(0);},
                "blue" => if num > 14 {return Ok(0);},
                _ => todo!(),
            }
        }
    }

    Ok(i)
}