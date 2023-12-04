use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count: i32 = 0;

    if let Ok(lines) = read_lines("./sample.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut characters = String::new();
                let mut digits = String::new();

                for c in ip.chars(){
                    characters.push(c);

                    match characters.as_str() {
                        "one" | "1" => digits.push('1'),
                        "two" | "2" => digits.push('2'),
                        "three" | "3" => digits.push('3'),
                        "four" | "4" => digits.push('4'),
                        "five" | "5" => digits.push('5'),
                        "six" | "6" => digits.push('6'),
                        "seven" | "7" => digits.push('7'),
                        "eight" | "8" => digits.push('8'),
                        "nine" | "9" => digits.push('9'),
                        _ => continue
                    }
                }

                if digits.len() >= 1 {
                    let mut number = String::new();
                    number.push(digits.chars().next().unwrap());
                    number.push(digits.chars().last().unwrap());
                    println!("{}", number);

                    count += number.parse::<i32>().unwrap();
                }
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