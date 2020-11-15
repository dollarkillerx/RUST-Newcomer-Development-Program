use std::mem;

fn main() {
    test1();
    test2();
    test3();
    test4();
}


fn apply<F>(f: F)
    where F: FnOnce() {
    f();
}

fn test1() {
    let greeting = "hello";

    let diary = || {
        println!("I said {}.", greeting);  // Fn
    };

    apply(diary);
}

fn test2() {
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();
    // let mut farewell = greeting.to_owned();

    let diary = || {
        farewell.push_str("cpx");

        println!("cpx: {}", farewell);
        mem::drop(farewell);
    };

    apply(diary);
}

fn func() {
    println!("I'm a function !");
}

fn call_me<F: Fn()>(f: F) {
    f()
}

fn test3() {
    call_me(func);
}

fn create_fn() -> impl Fn() {
    let text = "FN".to_owned();
    move || println!("This is a: {}", text)
}

fn test4() {
    let p = create_fn();
    p();
}