// collect
use std::process;
use std::collections::HashMap;
use std::env;

pub fn parse_params() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("./xxx query filepath");
        process::exit(1);
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    return (query, filename)
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> HashMap<usize, &'a str> {
    let mut results = HashMap::new();

    let insensitive = !env::var("CASE_INSENSITIVE").is_err();
    let mut rq = query.clone().to_string();
    if insensitive {
        rq = query.to_lowercase();
    }

    for (idx,line) in contents.lines().enumerate() {
        if insensitive {
            if line.to_lowercase().contains(rq.as_str()) {
                results.insert(idx, line);
            }
        }else {
            if line.contains(rq.as_str()) {
                results.insert(idx, line);
            }
        }
    }

    results
}