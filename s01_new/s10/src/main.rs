use std::rc::Rc;
use rand::prelude::*;

fn main() {
    // demo1();
    demo3();

    println!("{}","-".repeat(20));

    // 智能指针
    demo4();

    // Rc<T>
    // Rc::clone(&a) 增加引用计数
    // Rc::strong_count(&a) 获取强引用计数
    // Rc::weak_count(&a) 获取弱引用计数
}

fn demo1() {
    // 1. rust接受用户输入

    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // println!("你输入的内容是: {}", input);

    // 2. rust生成随机数

    // cargo add rand
    // let num = rand::thread_rng().gen_range(1, 101);
    // println!("生成的随机数是: {}", num);

    println!("猜猜我是谁");
    let num = thread_rng().gen_range(0
        ..=10);
    'iff: loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(r)=> {
                match r {
                    r if r.eq(&num) => {
                        println!("猜对了");
                        break 'iff;
                    },
                    r if r.gt(&num) => println!("猜大了"),
                    r if r.lt(&num) => println!("猜小了"),
                    _ => println!("请输入数字"),
                }
            },
            Err(_)=> println!("请输入数字"),
        }
    }
}

fn demo2() {
    #[derive(Clone, Debug, Copy)]
    struct User {
        // name: String,
        // 实现copy 必须所有的都实现copy， 任何不知道长度的类型都没有copy
        score: i32,
    }
}

fn demo3() {
    use s10::hello::*;
    hello_world();
}

fn demo4() {
    // Box, Rc,
    // Ref<T> & RefMut<T> & RefCell<T> 在运行时
    // 常见的: String, Vec, HashMap

    // 实现：
    // Deref trait: 允许智能指针struct的实例对象像引用一样使用
    // Drop trait:  当对象离开作用域时，自动调用自定义drop

    // 1. Box -> heap

    let mut b = Box::new(5); // heap
    println!("b = {}", b);
    *b += 10;
    println!("b = {}", b);


    #[derive(Debug)]
    enum List<T> {
        Nil,
        Cons(T, Box<List<T>>), // 在循环结构中  List 无法确定大小所有 要包裹box使他的大小固定
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("list = {:?}", list);

    // Rc<T> 引用计数器 可以rc被多个对象持有
    let name = Rc::new("tangzhe".to_string());
    #[derive(Debug)]
    struct User {
        name: Rc<String>,
    }

    let user = User {
        name: name.clone(),
    };

    println!("user = {:?}", user);

    let user2 = User {
        name: name.clone(),
    };

    println!("user2 = {:?}", user2);

    // RefCell<T> 内部可变性  当存在不可变引用的时候， 内部可变  （运行时）


}