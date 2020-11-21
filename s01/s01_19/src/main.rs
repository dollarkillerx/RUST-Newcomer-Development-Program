use std::cell::RefCell;

#[derive(Debug)]
struct User {
    name: RefCell<String>
}

fn main() {
    test_refcell();

}

fn test_refcell() {
    println!("Hello, world!");

    let u = User{
        name: RefCell::new("pcr".to_string()),
    };
    println!("{:?}",u);

    u.name.swap(&RefCell::new(String::from("pct")));
    println!("{:?}",u);

    u.name.borrow_mut().push_str("    hello world");

    println!("{:?}",u);
}