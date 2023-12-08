use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    
        let mut tot = 0;
        let mut num = String::new();
        let mut is_part = false;
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                let c = lines[y][x];
                if c.is_numeric() {
                    num.push(c);
                    if !is_part {
                        is_part = check_is_part(&lines, x as i32, y as i32);
                    }
                } else {
                    if is_part {
                        if let Ok(n) = num.parse::<usize>() {
                            tot += n;
                            dbg!(n);
                        }
                    }
                    num = String::new();
                    is_part = false;
                }
            }
        }

    println!("{}", tot);

    Ok(())
}

fn check_is_part (lines: &Vec<Vec<char>>, xb: i32, yb: i32) -> bool {
    let xmin: usize = 0.max((lines[0].len() as i32).min(xb-1)) as usize;
    let xmax: usize = 0.max((lines[0].len() as i32).min(xb+2)) as usize;
    let ymin: usize = 0.max((lines.len() as i32).min(yb-1)) as usize;
    let ymax: usize = 0.max((lines.len() as i32).min(yb+2)) as usize;

    for x_ in xmin..(xmax) {
        for y_ in ymin..(ymax) {
            if !lines[y_][x_].is_numeric() && lines[y_][x_] != '.' {
                return true;
            }
        }
    }

    return false;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_part() {
        let teststring = String::from("...\n...\n...\n");
        let mut lines: Vec<Vec<char>> = teststring
            .lines()
            .map(|l|
                l.to_owned()
                .chars()
                .collect()
            ).collect();

        assert_eq!(check_is_part(&lines, 1, 1), false);
        for y in 0..3 {
            for x in 0..3 {
                dbg!(x, y);
                lines[y][x] = '#';
                assert_eq!(check_is_part(&lines, 1, 1), true);
                lines[y][x] = '.';
            }
        }
    }

    // #[test]
    // fn test_2() {
    //     let file = File::open("input.txt").unwrap();
    //     // let reader = BufReader::new(file);
    //     let reader = String::from("111.\n...#");
    //     let lines: Vec<Vec<char>> = reader
    //         .lines()
    //         .map(|l| l.chars().collect())
    //         .collect();
    //     assert_eq!(check_is_part(&lines, 2, 0), true);
    // }

}