use std::collections::HashMap;

use adventofcode2023::helper::get_args;

fn main() {
    let (input, part) = get_args("day_03");

    let engine: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();
    let limits = (engine.len(), engine[0].len());

    let mut numbers = Vec::new();

    for (y, line) in engine.iter().enumerate() {
        let mut found_digit = false;
        let mut adjent = false;
        let mut adjent_star = None;
        let mut number = 0;
        for (x, c) in line.iter().enumerate() {
            if !c.is_digit(10) {
                if !found_digit { continue; }
                if adjent {
                    numbers.push((number, adjent_star));
                }
            
                adjent = false;
                found_digit = false;
                adjent_star = None;
                number = 0;
            } else {
                found_digit = true;
                number = (number * 10) + c.to_digit(10).unwrap();
                let r = check_adjent(&engine, &limits, x, y);
                if !adjent_star.is_some() { adjent_star = r.1 };
                adjent = adjent.max(r.0);
            }
        }
        if found_digit && adjent {
            numbers.push((number, adjent_star));
        }
    }

    if part == 1 {
        let sum = numbers.iter().fold(0u32, |sum, val| val.0 + sum);

        println!("sum: {}", sum);
    } else if part == 2 {
        let mut result: HashMap<Vec2, Vec<u32>> = HashMap::new();
        numbers.iter().for_each(|n| if let Some(p) = n.1 { result.entry(p).or_insert(vec![]).push(n.0); });
    
        let sum: u32 = result.iter().map(|(_, v)| v).filter(|v| v.len() == 2).map(|v| v.iter().fold(1u32, |r, n| r * n)).sum();
        println!("gear ratio sum: {}", sum);

    } else {
        eprintln!("undefined part: {}", part);
    }
}

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

fn check_adjent(engine: &Vec<Vec<char>>, limits: &(usize, usize), x: usize, y: usize) -> (bool, Option<Vec2>) {
    let on_right = x == (limits.0 - 1);
    let on_left = x == 0;
    let on_top = y == 0;
    let on_bottom = y == (limits.1 - 1);

    let mut adjent = false;
    let mut star_adjent_point = None;

    // checking
    // .#.
    // .c.
    // ...
    if !on_top {
        let c = engine[y - 1][x];
        adjent = adjent.max(is_symbol(c));
        if c == '*' { star_adjent_point = Some(Vec2 { x, y: y - 1 }); }
    }

    // checking
    // ...
    // .c.
    // .#.

    if !on_bottom {
        let c = engine[y + 1][x];
        adjent = adjent.max(is_symbol(c));
        if c == '*' { star_adjent_point = Some(Vec2 { x, y: y + 1 }); }
    }

    // checking
    // #..
    // #c.
    // #..
    if !on_left {
        // checking
        // #..
        // .c.
        // ...
        if !on_top {
            let c = engine[y - 1][x - 1];
            adjent = adjent.max(is_symbol(c));
            if c == '*' { star_adjent_point = Some(Vec2 { x: x - 1, y: y - 1 }); }
        }

        // checking
        // ...
        // #c.
        // ...
        let c = engine[y][x - 1];
        adjent = adjent.max(is_symbol(c));
        if c == '*' { star_adjent_point = Some(Vec2 { x: x - 1, y }); }

        // checking
        // ...
        // .c.
        // #..
        if !on_bottom {
            let c = engine[y + 1][x - 1];
            adjent = adjent.max(is_symbol(c));
            if c == '*' { star_adjent_point = Some(Vec2 { x: x - 1, y: y + 1 }); }
        }
    }

    // checking
    // ..#
    // .c#
    // ..#
    if !on_right {
        // checking
        // ..#
        // .c.
        // ...
        if !on_top {
            let c = engine[y - 1][x + 1];
            adjent = adjent.max(is_symbol(c));
            if c == '*' { star_adjent_point = Some(Vec2 { x: x + 1, y: y - 1 }); }
        }

        // checking
        // ...
        // .c#
        // ...
        let c = engine[y][x + 1];
        adjent = adjent.max(is_symbol(c));
        if c == '*' { star_adjent_point = Some(Vec2 { x: x + 1, y }); }

        // checking
        // ...
        // .c.
        // ..#
        if !on_bottom {
            let c = engine[y + 1][x + 1];
            adjent = adjent.max(is_symbol(c));
            if c == '*' { star_adjent_point = Some(Vec2 { x: x + 1, y: y + 1 }); }
        }
    }

    (adjent, star_adjent_point)
}

