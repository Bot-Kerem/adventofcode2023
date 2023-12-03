use std::collections::HashMap;

use adventofcode2023::helper::get_args;

fn main() {
    let (input, part) = get_args("day_01");

    let digits: HashMap<String, u32> = HashMap::from_iter(
        [
            ("one".to_owned(), 1),
            ("two".to_owned(), 2),
            ("three".to_owned(), 3),
            ("four".to_owned(), 4),
            ("five".to_owned(), 5),
            ("six".to_owned(), 6),
            ("seven".to_owned(), 7),
            ("eight".to_owned(), 8),
            ("nine".to_owned(), 9),
        ]
    );

    let sum = if part == 1 {
        input.lines().map(|line| {
            let list = line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    
            (list.first().unwrap() * 10) + list.last().unwrap()
        }).sum::<u32>()
    } else if part == 2 {
        input.lines().map(|line| {
            let line = line.chars().collect::<Vec<_>>();
            let mut i = 0;
            let mut list = Vec::new();
            'iterate_chars: while i < line.len() {
                if line[i].is_digit(10) {
                    list.push(line[i].to_digit(10).unwrap());
                } else {
                    for digit in &digits {
                        if line[i..].starts_with(&digit.0.chars().collect::<Vec<_>>()[..]) {
                            list.push(*digit.1);
                            i += digit.0.len() - 1;
                            continue 'iterate_chars;
                        }
                    }
                }
                i += 1;
            }
            (list.first().unwrap() * 10) + list.last().unwrap()
        }).sum::<u32>()
    } else {
        eprintln!("not valid part");

        std::process::exit(1);
    };

    println!("Sum of all of the calibration values is: {}", sum);


}

