use std::env;
use std::fs;

enum Args {
    InputPath,
    Part,
}
pub fn get_args(default: &str) -> (String, u32) {
    let mut input_path = format!("input/{}.txt", default);
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

    (input, part)
}


