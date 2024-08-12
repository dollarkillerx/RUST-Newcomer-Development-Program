fn main() {
    // 本章内容: 1.流程控制, 2.函数 3.借用

    // 1.流程控制
    // if switch
    // for while do-while
    // break continue goto
    demo1();
    println!("{}","-".repeat(30));
    demo2();
    // 2.函数
    println!("{}","-".repeat(30));
    demo3();

    // 3.借用
    println!("{}","-".repeat(30));
    demo4();

}

fn demo1() {
    let age = 50;
    let can_vote = if age >= 18 { true } else { false };
    println!("can vote: {}", can_vote);

    match age {
        0..=18 => println!("too young"),
        19..=35 => println!("good age"),
        _ => println!("too old"),
    }

    match can_vote {
        true => println!("yes"),
        false => println!("no"),
    }

    match age {
        25|26 => println!("25 or 26"),
        _ => println!("other age"),
    }

    match age {
        x if x < 60 => println!("{} years old", x),
        _ => println!("too old"),
    }

    let can_vote = match age {
        0..=18 => false,
        _ => true,
    };
    println!("can vote: {}", can_vote);
}

fn demo2() {

    for i in 0..10 {
        println!("{}", i);
    }

    for i in 0..=10 {
        println!("{}", i);
    }

    loop {
        println!("hello");
        break;
    }

    'outer: loop {
        println!("hello");
        'inner: loop {
            println!("hello");
            break 'outer;
        }
    }

    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    // 迭代器
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("{}", i);
    }

    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    for i in arr {
        println!("{}", i);
    }

    // map 去做一些事情
    arr.iter().map(|i| i * 2).for_each(|i| {
        println!("{}", i);
    });

    // filter 过滤不满足的条件
    arr.iter().filter(|i|i > &&2).for_each(|i| {
        println!("{}", i);
    });
    arr.iter().cloned().filter(|i| i > &2).for_each(|i| {
        println!("{}", i);
    });
    arr.iter().map(|&x| x).filter(|i| i > &2).for_each(|i| {
        println!("{}", i);
    });

    println!("{}", "-".repeat(10));
    // collect
    let arr = arr.iter().map(|i| i * 2).filter(|i| i > &2).collect::<Vec<i32>>();
    println!("{:?}", arr);

    // Vec<_> 会自动推导
    let arr:Vec<_> = arr.iter().map(|&x| x * x).collect();
    println!("{:?}", arr);

    println!("{}", "-".repeat(10));
    // reduce
    let sum = arr.iter().fold(0, |a, b| a + b);
    println!("{}", sum);

}

fn demo3() {
    let mut a = 16;

    fn change_i32(mut x: i32) {
        x = x * 2; // 这里是copy 这里的改变不会影响到外面
        println!("X : {}", x);
    }
    fn change_i32_mut(x: &mut i32) {
        *x = *x * 2; // 这里是mut 这里的改变会影响到外面
        println!("X : {}", x);
    }
    change_i32(a);
    println!("a : {}", a);

    change_i32_mut(&mut a);
    println!("a : {}", a);
}

fn demo4() {
    fn move_fn(p1: i32, p2: String) {
        println!("p1 : {}", p1); // clone
        println!("p2 : {}", p2); // move
    }

    fn mut_fn(p1: i32, p2: &str) {
        println!("p1 : {}", p1); // clone
        println!("p2 : {}", p2); // 借用
    }

    fn mut_fn2(p1: &mut i32, p2: &mut String) {
        *p1 += 20; // 基础类型必须手动取引用
        (*p2).push_str(" world");
        p2.push_str(" world"); // 自动取引用
    }

    let p1 = 10;
    let p2 = String::from("hello");
    move_fn(p1, p2);
    println!("p1 : {}", p1);
    // println!("p2 : {}", p2); // move
    let p1 = 10;
    let p2 = String::from("hello");
    mut_fn(p1, &p2);
    println!("p1 : {}", p1);
    println!("p2 : {}", p2);

    let mut p1 = 10;
    let mut p2 = String::from("hello");
    mut_fn2(&mut p1, &mut p2);
    println!("p1 : {}", p1);
    println!("p2 : {}", p2);


}