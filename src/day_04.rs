use adventofcode2023::helper::get_args;

#[derive(Debug)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    hand: Vec<u32>,
}

enum Expect {
    Id,
    WinningNuber,
    Hand,
}

fn main() {
    let (input, part) = get_args("day_04");

    let tokens = lex(&input);

    let mut games = Vec::new();
    let mut expect = Expect::Id;

    for token in tokens {
        match token.as_str() {
            "Card" => {
                games.push(Game { id: 0, winning_numbers: vec![], hand: vec![] });
                expect = Expect::Id;
            },
            "|" => expect = Expect::Hand,
            ":" => expect = Expect::WinningNuber,
            _ => match expect {
                Expect::Id => games.last_mut().unwrap().id = token.parse::<u32>().unwrap(),
                Expect::WinningNuber => games.last_mut().unwrap().winning_numbers.push(token.parse::<u32>().unwrap()),
                Expect::Hand => games.last_mut().unwrap().hand.push(token.parse::<u32>().unwrap()),
            }
        }
    }

    if part == 1 {
        let total_points = games.iter().map(|g| {
            g.hand.iter().filter(|&n| g.winning_numbers.contains(n)).collect::<Vec<&u32>>()
        }).filter(|c| !c.is_empty()).fold(0u32, |sum, num| sum + 2u32.pow((num.len() - 1) as u32));

        println!("total points: {}", total_points);
    } else if part == 2 {
        println!("undone");
    } else {
        eprintln!("undefined part: {}", part);
    }
}

fn lex(input: &String) -> Vec<String> {
    let mut elems = Vec::new();
    let mut buf = String::new();
    input.chars().for_each(|c| {
        if c.is_whitespace() || c == ':' || c == '|' || c == '\n' {
            if !buf.is_empty() {
                elems.push(buf.clone());
                buf.clear();
            }
            if !c.is_whitespace() && c != '\n' {
                elems.push(c.to_string());
            }
        } else if c.is_digit(10) || c.is_alphabetic() {
            buf.push(c)
        } else {
            println!("unknown character: {}", c);
        }
    });

    if !buf.is_empty() {
        elems.push(buf)
    }

    elems
}

