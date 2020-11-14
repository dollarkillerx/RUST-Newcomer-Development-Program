fn main() {
    loop_t();
    loop_t2();

    t2();
}

fn loop_t() {
    let mut b = 0;
    'a:
    loop {
        println!("a");
        loop {
            if b < 10 {
                b += 1;
            } else {
                break 'a;
            }
        }
    }
}

fn loop_t2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result : {}", result);
}

fn t2() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatterm kaboom!"),
        _ => println!("fuck you"),
    };
}