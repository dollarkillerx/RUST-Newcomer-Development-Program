use tr::Summary;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let w = Weather { weather: "sss".to_string() };
    println!("summary: {}", w.summarize());

    let c = 12;
    println!("c: {}", c.summarize());

    notify(c);
    notify2(c);

    test1();
    test2();
    test3();
}

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

struct Weather {
    weather: String
}

impl Summary for Weather {
    // overwrite
    fn summarize(&self) -> String {
        self.weather.clone()
    }
}

fn notify(item: impl Summary) {
    println!("notify: {}", item.summarize());
}

fn notify2<T: Summary>(item: T) {
    println!("notify2: {}", item.summarize());
}

fn test1() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = longest(str1.as_str(), str2);
    println!("the longest string is {}", result);

    let result = longest2(str1.as_str(), str2);
    println!("the longest2 string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test2() {
    let novel = String::from("Call me Ishmael. Some years ago ...");

    let first_sentence = novel.split('.')
        .next()
        .expect("Could not fund a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("i: {:?}", i);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
{
    println!("announcement! {}", ann);
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

fn test3() {
    println!("r: {}",longest_with_an_announcement("abc","bs","sad"));
}