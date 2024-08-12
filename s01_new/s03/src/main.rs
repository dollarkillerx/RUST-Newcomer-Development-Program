use std::collections::HashMap;

fn main() {
    // 本章 主要: 内存管理模型， String &str ownership
    demo1();
    println!("{}", "-".repeat(20));
    demo2();
    println!("{}", "-".repeat(20));
    demo3();
    println!("{}", "-".repeat(20));
    demo4();
    println!("{}", "-".repeat(20));
    stack_heap();
}

fn demo1() {
    let s1 = String::from("s1");
    let s2 = s1.clone(); // 不用clone 默认move
    println!("s1: {}, s2: {}", s1, s2);

    let s3 = s1;
    println!(" s3: {}", s3); // s1 转移了所有权move 已经无了
    get_length(s3);
    // println!(" s3: {}", s3); // s3 转移了所有权move 已经无了

    get_length2(&s2); // 使用引用
    println!(" s2: {}", s2);

    let j = dangle_static();
    println!("j: {}", j);

    let hello = "hello world";
    let word = first_word(hello); // 切片引用
    println!("word: {}", word);
    println!("hello: {}", hello);
}

fn get_length(s :String) {
    println!("length: {}", s.len()); // s 转移了所有权move 已经无了
}


fn get_length2(s :&str) {
    println!("length: {}", s.len()); // 没有转移所有权
}

// static
fn dangle_static() -> &'static str {
    "hello"
}

// 字符串切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn demo2() {
    let name = String::from("value c++");
    // String::from
    // to_string()
    // to_owned()
    let course = "Rust".to_owned(); // &str => String
    println!("new: {}, course: {}", name, course);
    let new_name = name.replace("c++","CPP");
    println!("name: {}, new_name: {}", name,new_name);
}

struct Person {
    name: String,
    age: u8
}

struct Person2<'a> {
    name: &'a str,  // 指定name的生命周期与Person2相同
}


enum Color {
    Red,
    Blue,
    Green
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green")
    }
}

enum BuildingLocation {
    Building(String, i32),
    Office(String, i32)
}

impl BuildingLocation {
    // 关联函数
    fn print_location(&self) {
        match self {
            BuildingLocation::Building(name, floor) => println!("{}: {}", name, floor),
            BuildingLocation::Office(name, floor) => println!("{}: {}", name, floor)
        }
    }
}

fn demo3() {
    let color = Color::Blue;
    print_color(color);
    let b1 = BuildingLocation::Building(String::from("b1"), 1);
    b1.print_location();
    let b2 = BuildingLocation::Office(String::from("b2"), 2);
    b2.print_location();
}

// 口味
#[derive(Debug)]
enum Flavor {
    Spicy, // 麻辣
    Sweet, // 甜
    Sour // 咸
}

// 饮料
#[derive(Debug)]
struct Drink {
    flavor: Flavor, // 口味
    price: f64, // 单价
    is_iced: bool // 是否加冰
}

impl Drink {
    fn get_price(&self) -> f64 {
        if self.is_iced {
            self.price * 0.8
        } else {
            self.price
        }
    }

    fn print_drink(&self) {
        println!("{:?}", self);
    }
}

fn print_drink(d: Drink) {
    println!("{:?}", d);
}

struct User {
    user: String,
    balance: f64,
}

impl User {
    fn new(user: String, balance: f64) -> User {
        User {
            user,
            balance
        }
    }
}

struct Bank {
    bank: HashMap<String, User>
}

impl Bank {
    fn new() -> Bank {
        Bank {
            bank: HashMap::new()
        }
    }
    fn add_user(&mut self, user: User) {
        self.bank.insert(user.user.clone(), user);
    }
    fn remove_user(&mut self, user: &str) {
        self.bank.remove(user);
    }
    fn loop_users(&self) {
        for (k, v) in self.bank.iter() {
            println!("user: {}, balance: {}", k, v.balance);
        }
    }
}

fn demo4() {
    let drink = Drink {
        flavor: Flavor::Spicy,
        price: 3.0,
        is_iced: true
    };
    // print_drink(drink); // 所有权会转移
    drink.print_drink(); // 不会转移

    // 借用 与 可变借用
    let user = User::new(String::from("user1"), 100.0);
    let user2 = User::new(String::from("user2"), 102.0);
    let mut bank = Bank::new();
    bank.add_user(user);
    bank.add_user(user2);
    bank.loop_users();
}

fn stack_heap() {
 // stack 栈
    // 1. 堆栈将按照先进后出的顺序来分配内存  并以相反的顺序来释放内存
    // 2. 堆栈只能存放基本类型  不能存放引用
    // 3. 栈内存是有限的  无法存放更多的数据
    // 4. 堆栈上存储的所有数据必须具有已知的固定大小
 // heap 堆
    // 1. 堆内存是无限的  无法存放更多的数据
    // 2. 堆内存可以存放引用
    // 3. 长度不确定

    // Box 堆指针

    // Move: 所有权转移
    // Clone: 深拷贝
    // Copy: 在Clone的基础上，建立的maker trait
    // trait: 定义共享行为



    // rust 中的 stack heap
    // stack 默认行为copy, 但是struct 是move
        // 1. 基础类型
        // 2. tuple and array
        // 3. struct 与 enum ， 如果属性有String等在heap上的数据类型 会指向堆内存
    // heap 默认行为move
        // Box Rc String/Vec

    struct Point {
        x: f64,
        y: f64
    }

    // 创建在堆上
    let mut boxed_point = Box::new(Point { x: 1.0, y: 4.0 });
    println!("x: {}, y: {}", boxed_point.x, boxed_point.y);
    boxed_point.x += 1.1;
    println!("x: {}, y: {}", boxed_point.x, boxed_point.y);

    // 创建在栈上
    let mut stack_point = Point { x: 1.0, y: 4.0 };
    println!("x: {}, y: {}", stack_point.x, stack_point.y);
    stack_point.x += 1.2;
    println!("x: {}, y: {}", stack_point.x, stack_point.y);

    let mut box_int = Box::new(1);
    println!("box_int: {}", box_int);
    *box_int += 1;
    println!("box_int: {}", box_int);

    let mut stack_int = 1;
    println!("stack_int: {}", stack_int);
    stack_int += 1;
    println!("stack_int: {}", stack_int);

    let x_vec = vec![1, 2, 3];
    let y_vec = x_vec;
    // println!("x_vec: {:?}", x_vec); // move
    println!("y_vec: {:?}", y_vec);

    let xr = "hello";
    let yr = xr; // &str copy
    let yr2 = xr.to_string();
    println!("yr: {}", yr);
    println!("xr: {}", xr);
    println!("yr2: {}", yr2);

    // 如果struct所有的类型都为基础类型 则存储在栈上 但是默认行为是move
    #[derive(Debug,Clone,Copy)] // 默认行为是copy
    struct Book {
        page: i32,
        rating: f64,
    }

    let b1 = Book { page: 100, rating: 3.5 };
    let b2 = b1; // copy clone
    let b3 = b1; // copy clone
    println!("b1: {:?}", b1);
    println!("b2: {:?}", b2);
    println!("b3: {:?}", b3);
}