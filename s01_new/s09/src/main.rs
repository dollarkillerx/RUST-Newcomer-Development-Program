fn main() {
    // 本章内容: 闭包
    demo1();
    println!("{}", "-".repeat(30));
    // 不可变 Fn
    // 可变 FnMut
    // move FnOnce 不适用move关键字编译器会自动推导是否实现move
    demo2();
}

fn demo1() {
    #[derive(Debug)]
    struct User {
        name: String,
        score: i32,
    }

    let user = User {
        name: String::from("tangzhe"),
        score: 99,
    };

    let user2 = User {
        name: String::from("tangzhe2"),
        score: 70,
    };

    let user3 = User {
        name: String::from("tangzhe3"),
        score: 20,
    };

    let user4 = User {
        name: String::from("tangzhe4"),
        score: 10,
    };

    let mut users = vec![
        user, user2, user3, user4
    ];

    users.sort_by_key(|x| x.score);

    println!("{:?}", users);
}

fn demo2() {
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let fn_func = |s| {
        println!("{s1}");   // 没有修改s1 编译器默认为不可变引用
        println!("i am {s}");
    };
    fn_func(s2.clone());
    fn_func("hello world".to_owned());

    println!("{}", "-".repeat(10));


    let mut s1 = String::from("hello");
    let s2 = String::from(" world");
    let mut fn_func = |s| {
        println!("{s1}");   // 没有修改s1 编译器默认为不可变引用
        println!("i am {s}");
        s1.push_str("!!!"); // 修改 默认&mut 可变引用
        println!("{s1}");
    };
    fn_func(s2.clone());
    fn_func("hello world".to_owned());

    println!("{}", "-".repeat(10));

    let mut s1 = String::from("hello");
    let mut fn_func = move || {
        s1.push_str("!!!");
        println!("{s1}");
    };
    fn_func(); // hello!!!
    fn_func(); // hello!!!!!!
    // 闭包 fn_func 可以被调用两次并打印 s1，这是因为闭包在第一次调用时并没有消耗掉自己
    // println!("{s1}");  // move

}