use std::fmt::Formatter;

fn main() {
    // 本章内容:
    //  1.函数返回值与所有权
    //  2.高阶函数 函数作为参数与返回值
    //  3.错误处理
    demo1();
    println!("{}", "-".repeat(30));
    demo2();
    println!("{}", "-".repeat(30));

    // 使用？会吧异常抛出
    match demo3() {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    };
}

fn demo1() {
    // 返回在堆栈 还是 栈

    fn new_name() -> String {
        String::from("hello") // 返回 在栈
    }
    fn new_age() -> i32 {
        18 // 返回 在堆栈
    }

    let name = new_name();
    println!("{}", name);

    let age = new_age();
    println!("{}", age);
}

fn demo2() {
    // 使用最多的高阶函数
    // map， filter, fold(迭代作用与每个元素上)

    // map
    let v = vec![1, 2, 3];
    let v2 = v.iter().map(|&x| x + 1).collect::<Vec<i32>>();
    println!("{:?}", v2);

    let v = vec![1, 2, 3];
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    // filter
    let v = vec![1, 2, 3];
    let v2 = v.iter().map(|&x|x).filter(|&x| x > 1).collect::<Vec<i32>>();
    println!("{:?}", v2);

    let v = vec![1, 2, 3];
    let v2:Vec<_> = v.iter().map(|x|x).filter(|x| x > &&1).collect();
    println!("{:?}", v2);

    let v = vec![1, 2, 3];
    let v2:Vec<_> = v.iter().filter(|x| x > &&1).collect();
    println!("{:?}", v2);

    let v = vec![1, 2, 3];
    let v2:Vec<_> = v.into_iter().filter(|&x| x > 1).collect();
    println!("{:?}", v2);

    // // fold
    let v = vec![1, 2, 3];
    let v2 = v.iter().fold(0, |acc, x| acc + x);
    println!("{}", v2);
}



// 如果使用？抛出异常当前fn必须有返回  Result<(), Box<dyn std::error::Error>>
fn demo3() -> Result<(), Box<dyn std::error::Error>> {
    // Result, Option, panic!
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("除数不能为0"))
        } else {
            Ok(a / b)
        }
    }

    let r = divide(10, 0);
    match r {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }

    fn find_element(v: Vec<i32>, x: i32) -> Option<i32> {
        for (index, value) in v.iter().enumerate() {
            if *value == x {
                return Some(index as i32);
            }
        }
        None
    }

    let v = vec![1, 2, 3];
    let r = find_element(v, 2);
    match r {
        Some(v) => println!("{}", v),
        None => println!("None"),
    }

    // panic!("程序崩溃了");


    // unwrap() and ?

    let v = vec![1, 2, 3];
    // let r = divide(10, 0).unwrap(); // 如果返回 Err 会 panic
    // println!("{}", r);
    // 使用？会吧异常抛出
    let r = divide(10,0)?;
    println!("{}", r);


    // 自定义错误
    #[derive(Debug)]
    struct MyError {
        detail: String,
    }
    // 实现Error trait
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError: {}", self.detail)
        }
    }
    impl std::error::Error for MyError {
        fn description(&self) -> &str {
            &self.detail
        }
    }

    fn divide2(a: i32, b: i32) -> Result<i32, MyError> {
        if b == 0 {
            Err(MyError { detail: String::from("除数不能为0") })
        } else {
            Ok(a / b)
        }
    }
    let r = divide2(10, 0);
    match r {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }


    Ok(())
}