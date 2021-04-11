use mimi_grep::*;
use std::fs;
use std::process;

fn main() {

    let (query, filepath) = parse_params();

    let contents = fs::read_to_string(filepath)
        .unwrap_or_else(|err| {
            println!("Something went wrong reading the file, err: {:?}", err);
            process::exit(1);
        });

    let r = search_case_insensitive(&query, &contents);
    println!("{:#?}", r);
}

