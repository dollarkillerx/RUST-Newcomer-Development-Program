// 每个值都有一个变量, 这个变量是该值的所有者
// 每个值同时只能有一个所有者
// 当所有者超出作用域(scope)时，该值会被删除

fn main() {
    // test1();
    // let c = gives_ownership();
    // println!("c: {}",c);
    //
    // let p = String::from("hello world");
    // pri(&p);
    // println!("p: {}", p);

    let p = "hello world";
    let c = first_worlds(p);
    println!("c: {}",c);
    println!("p: {}",p);

    let p = String::from("hello world");
    let c = first_worlds(&p);
    println!("c2: {}",c);
    println!("p2: {}",p);
}

fn pri(r: &str) {
    println!("r: {}, r len: {}", r, r.len());
}

fn test1() {
    let mut hello_world= String::from("Hello");
    hello_world.push_str(", world");
    println!("{}", hello_world);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn first_worlds(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}


// fn first_worlds(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i]
//         }
//     }
//     &s[..]
// }

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}