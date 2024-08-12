fn main() {
    // 本章内容: mut type 常用类型
    mut_study();
    const_study();
    basic_data_types();
    types_study();
    ownership_study();
}

fn mut_study() {
    // 1. mut
    let name = "zhangsan";
    // name="lisi"; // 报错  rust 默认不允许变量重新赋值
    println!("name = {}", name);
    let mut name = "zhangsan";
    println!("name = {}", name);
    name = "lisi";
    println!("name = {}", name);

    let x:i32 = 5;
    let mut y = String::from("hello");
    println!("y = {}", y);
    {
        let x:&str=  "hello world";
        println!("x = {}", x);
        y.push_str(" world");
        // 在这个作用域结束的时候 x被销毁
    }
    println!("x = {}", x);
    println!("y = {}", y);
}

fn const_study() {
    const SECOND_HOUR: usize = 3600;
    println!("SECOND_HOUR = {}", SECOND_HOUR);

    const SECOND_DAY: usize = SECOND_HOUR * 24;
    println!("SECOND_DAY = {}", SECOND_DAY);

    // 写入代码段
}


// Basic data types
fn basic_data_types() {
    // int: {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize}
    // float: {f32, f64}
    // bool
    // char
    // string
    println!("i32 max: {}", i32::MAX);
    println!("i32 min: {}", i32::MIN);
    println!("u32 max: {}", u32::MAX);
    println!("u32 min: {}", u32::MIN);

    println!("u32 : {}", std::mem::size_of::<u32>());
    println!("i32 : {}", std::mem::size_of::<i32>());
}

fn types_study() {
    // tuples, arrays, map
    // tuple 类型可以不同
    let tup = (1, 2, 3, "张三");
    println!("tup = {:?}", tup);
    // 常用tuple 方法
    println!("tup.3 = {}", tup.3);

    let (a, b, c, d) = tup;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    let (a, b, c, _) = tup;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let (a, ..) = tup;
    println!("a = {}", a);

    let (a, b, ..) = tup;
    println!("a = {}, b = {}", a, b);

    let (a, .., c) = tup;
    println!("a = {}, c = {}", a, c);

    let (a, .., c, d) = tup;
    println!("a = {}, c = {}, d = {}", a, c, d);

    // array 类型必须相同
    let arr = [1, 2, 3];
    println!("arr = {:?}", arr);
    // 常用tuple 方法
    println!("arr[0] = {}", arr[0]);

    // 常用array 方法
    println!("arr.len() = {}", arr.len());

    // map
    let mut map = std::collections::HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("map = {:?}", map);
}

fn ownership_study() {
    let tup = (0,"hi",3.4);
    println!("tup = {:?}", tup);
    let (a, b, c) = tup;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let str_item = String::from("aa");
    let str_item_2 = str_item;
    // println!("str_item = {}", str_item); // move 转移了所有权 str_item不存在了
    println!("str_item2 = {}", str_item_2);

    let aj = 1;
    let ab = aj;
    println!("ab = {}", ab);
    println!("aj = {}", aj); // 基础数据类型默认clone 所以还存在
}