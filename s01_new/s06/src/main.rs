fn main() {
    // 本章内容: borrowing借用 && Lifetime生命周期
    // ## 不可变引用和可变引用
    // 可变引用和不可变引用不可以同时存在
    // 同一时间内只能有一个可变引用
    // 可以有多个不可变引用

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3); // 同一时间内只能有一个可变引用
    print!("{}", r3);

    let result: &str;
    {
        result = "hello"; // &'static str
        println!("{}", result);
    }
    println!("{}", result);


    // ## 生命周期
    // 每个引用都有一个生命周期
    // 如果有一个输入生命周期参数 那么该生命周期将被分配给所有输出生命周期参数
    // 如果多个输入生命周期参数，其中一个是对self或不可变self的引用时。self的生命周期将被分配给所有输出生命周期参数

    fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


    let r1 = String::from("hello");
    let r2 = String::from("world");
    let result = longest(&r1, &r2);
    println!("{}", result);

    fn longest2<'a, 'b, 'out>(x: &'a str, y: &'b str) -> &'out str
    where
        'a: 'out,
        'b: 'out,
    {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let r1 = String::from("hello");
    let r2 = String::from("world");
    let result = longest2(&r1, &r2);
    println!("{}", result);

    // 结构体中的引用 必须有生命周期
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
        fn get_part (&self) -> &str {
            self.part
        }
    }

    let r1 = String::from("hello");
    let r2 = String::from("world");
    let r3 = ImportantExcerpt {
        part: &r1
    };
    println!("{}", r3.get_part());
}
