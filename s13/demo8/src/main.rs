use demo8::*;

#[derive(Debug, Default, Clone)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 0,
            active: false,
        }
    }
    fn string(&self) -> String {
        format!("{:#?}", &self)
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let d = Dim(46, 89);
    println!("dim 46 89 area: {}", d.area());

    println!("==================");
    let c = a_big(16);
    println!("{:?}", c);
    let c = a_big(160);
    println!("{:?}", c);

    front_of_house::awitlist();
}

#[derive(Debug)]
struct Dim(u32, u32);

impl Dim {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
}

fn test1() {
    println!("Hello, world!");

    let mut u = User::default();
    u.username = String::from("Dollarkiller");
    u.email = String::from("dollarkiller@dollarkiller.com");
    let c = User {
        sign_in_count: 666,
        active: true,
        ..(u.clone()) // value borrowed here after partial move
    };

    println!("{}", u.string());
    println!("{}", c.string());

    let black = Color(0, 0, 0);
    println!("{:#?}", black);
}

enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IPAddrKind {
    fn v4(v4: (u8, u8, u8, u8)) -> Self {
        Self::V4(v4.0, v4.1, v4.2, v4.3)
    }
    fn v6(addr: String) -> Self {
        Self::V6(addr)
    }
}


fn a_big(a: i32) -> Option<i32> {
    if a > 100 {
        return Some(a);
    }

    None
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn val_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}