use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut characters = String::new();
                let mut digits: String = String::new();


                for c in ip.chars(){
                    characters.push(c);

                    match characters.as_str() {
                        x if characters.contains("one") => push_and_delete(&mut characters, &mut digits, "one".to_string(), '1'),
                        x if characters.contains("two") => push_and_delete(&mut characters, &mut digits, "two".to_string(), '2'),
                        x if characters.contains("three") => push_and_delete(&mut characters, &mut digits, "three".to_string(), '3'),
                        x if characters.contains("four") => push_and_delete(&mut characters, &mut digits, "four".to_string(), '4'),
                        x if characters.contains("five") => push_and_delete(&mut characters, &mut digits, "five".to_string(), '5'),
                        x if characters.contains("six") => push_and_delete(&mut characters, &mut digits, "six".to_string(), '6'),
                        x if characters.contains("seven") => push_and_delete(&mut characters, &mut digits, "seven".to_string(), '7'),
                        x if characters.contains("eight") => push_and_delete(&mut characters, &mut digits, "eight".to_string(), '8'),
                        x if characters.contains("nine") => push_and_delete(&mut characters, &mut digits, "nine".to_string(), '9'),
                        x if characters.contains("1") => push_and_delete(&mut characters, &mut digits, "1".to_string(), '1'),
                        x if characters.contains("2") => push_and_delete(&mut characters, &mut digits, "2".to_string(), '2'),
                        x if characters.contains("3") => push_and_delete(&mut characters, &mut digits, "3".to_string(), '3'),
                        x if characters.contains("4") => push_and_delete(&mut characters, &mut digits, "4".to_string(), '4'),
                        x if characters.contains("5") => push_and_delete(&mut characters, &mut digits, "5".to_string(), '5'),
                        x if characters.contains("6") => push_and_delete(&mut characters, &mut digits, "6".to_string(), '6'),
                        x if characters.contains("7") => push_and_delete(&mut characters, &mut digits, "7".to_string(), '7'),
                        x if characters.contains("8") => push_and_delete(&mut characters, &mut digits, "8".to_string(), '8'),
                        x if characters.contains("9") => push_and_delete(&mut characters, &mut digits, "9".to_string(), '9'),
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

fn push_and_delete(characters: &mut String, digits: &mut String, text: String, number: char) {
    let mut new_text = text.clone();
    new_text.remove(0);
    *characters = characters.replacen(&text, &new_text, 1);
    digits.push(number);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}