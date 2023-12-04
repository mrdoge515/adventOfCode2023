use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let digits = ip.replace(|c: char| !c.is_ascii_digit(), "").to_owned();
                let mut number = String::new();

                number.push(digits.chars().next().unwrap());
                number.push(digits.chars().last().unwrap());
                
                count += number.parse::<i32>().unwrap();
            }
         }
    }

    println!("{}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}