fn main() {
    mut_test();
    str_test();
    test2();
}

fn mut_test() {
    let b = Box::new(1212);
    let mut c = b;  // 当所有权转移时，数据的可变性可能发生改变
    *c = 213213;

    println!("c: {}", c);
}

#[derive(Debug)]
struct P {
    name: String,
}

fn str_test() {
    // let mut t = P{name:"pc".to_string()};
    let mut t = P { name: "pc".parse().unwrap() };

    str_t1(&mut t);

    println!("t: {:?}", t);
}

fn str_t1(s: &mut P) {
    // s.name.push_str("cpx");
    let c = "sp".to_string();
    s.name += c.as_str();
    println!("{}", s.name);
}

fn test2() {
    let mut mutable_integer = 7i32;

    {
        // 借用 `_mutable_integer`
        let large_integer = &mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // mutable_integer = 50;
        // 改正 ^ 注释掉此行

        println!("Immutably borrowed {}", large_integer);

        // `large_integer` 离开作用域
    }

    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    mutable_integer = 3;
    println!("Immutably borrowed {}", mutable_integer);
}