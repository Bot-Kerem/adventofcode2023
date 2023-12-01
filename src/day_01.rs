use std::{fs, env, collections::HashMap};

enum Args {
    InputPath,
    Part,
}

fn main() {
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

    let mut input_path = "input/day_01.txt".to_owned();
    let mut part = 1;
    let mut arg_next: Option<Args> = None;
    for arg in env::args() {
        if let Some(next) = arg_next {
            match next {
                Args::InputPath => input_path = arg.clone(),
                Args::Part => part = arg.parse::<u32>().unwrap(),
            }
            arg_next = None;
        }
        if arg == "-p" || arg == "--part" {
            arg_next = Some(Args::Part);
        } else if arg == "-i" || arg == "--input_path" {
            arg_next = Some(Args::InputPath);
        }
    }
    let input = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("failed to open file error: {:?}", err);

        std::process::exit(1);
    });

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

