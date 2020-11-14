use std::fmt;
use std::fmt::{Debug, Display};

fn main() {
    hello("dollarkiller".parse().unwrap());
    print_all();
    print_struct();
    print_person();
    print_minmax();
    print_list();
}

/// hello world
///
/// # params:
/// * `name` -  person name
///
/// # example:
///
/// ```
///  hello("dollarkiller".parse().unwrap());
/// ```
fn hello(name: String) {
    println!("hello {}", name);
}

// cargo doc --open 生成文档注释

fn print_all() {
    println!("{} days", 32); // out stdio
    eprintln!("{} days", 32); // out stderr

    println!("{0}, this is {1}, {1}, this is {0}", "t0", "t1");


    let title = format!("{sub} {verb} object={out}", sub = "sub1", verb = "verb1", out = "outc");
    println!("{}", title);
}

#[derive(Debug)]
struct User {
    name: String,
    age: i8,
}

impl User {
    fn new(name: String, age: i8) -> User {
        User { name, age }
    }
}

fn print_struct() {
    let c = User::new("dollarkiller".to_string(), 18);
    println!("{:?}", c);
    println!("{:#?}", c); // # format
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: u8) -> Person {
        Person {
            name,
            age,
        }
    }
    fn print(&self) {
        println!("name: {}, age: {}", self.name, self.age);
    }
}

fn print_person() {
    let name = "dollarkiller";
    let p = Person::new(name, 18);
    p.print();
}

struct MinMax(i64, i64);

impl MinMax {
    fn new(i: i64) -> MinMax {
        MinMax(i, 12)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

fn print_minmax() {
    let c = MinMax::new(12);
    println!("{}", c);
}


struct List<T>(Vec<T>);

impl<T:Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?
        }

        write!(f, "]")
    }
}

fn print_list() {
    let l = List(vec![1, 2, 3, 3, 3, 4]);
    println!("{}", l);
}

