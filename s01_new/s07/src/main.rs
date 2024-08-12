fn main() {
    // 本章内容: 1. generic繁星 2. trait
    demo1();
    println!("{}", "-".repeat(30));
    demo2();
    println!("{}", "-".repeat(30));
    trait_obj();
    println!("{}", "-".repeat(30));
    trait_obj_pro(); // trait obj 和 泛型
}

fn demo1() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        // fn new(x: T, y: T) -> Self {
        //     Point { x, y }
        // }
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_to_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("{}", p.x());

    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_to_origin());
    println!("{}", p.x());


    fn swap<T>(a: T, b: T) -> (T, T) {
        (b, a)
    }

    let a = "hello world".to_owned();
    let b = "hello rust".to_owned();
    let (b, a) = swap(a, b);
    println!("{}", a);
    println!("{}", b);
}

fn demo2() {
  // trait
    // 1. 内置常量: 整个程序的生命周期
    // 2. 默认实现
    // 3. 多重实现
    // 4. 特质边界
    // 5. Trait Alias

    trait Greeter {
        fn greet(&self);
        fn hello_world() {
            println!("hello, world");
        }
        fn hello(&self) {
            println!("hello, world");
        }
    }
    struct Person{
        name: String
    }

    // Person 实现 Greeter
    impl Greeter for Person {
        fn greet(&self) {
            println!("hello, {}", self.name);
        }
    }

    let person = Person{name: "rust".to_owned()};
    person.greet();
    Person::hello_world();
    person.hello();
}

fn trait_obj() {
    // 运行时泛型
    // dyn
    // 不可变引用
        // &dyn Trait
    // 可变引用 (使用非常少)
        // &mut dyn Trait
    // move   (大量使用)
        // Box<dyn Trait>

    // 1. trait 不可变引用
    struct Obj{}
    trait Overview {
        fn overview(&self) -> String {
            String::from("trait Overview")
        }
    }

    impl Overview for Obj {
        fn overview(&self) -> String {
            String::from("impl Overview for Obj")
        }
    }
    fn call_obj(item:&impl Overview) { // 不可变引用
        println!("{}", item.overview());
    }

    let obj = Obj{};
    call_obj(&obj);
    println!("{}",obj.overview());

    // 2. trait 可变引用

    // 3. trait move
    fn call_obj_box(item: Box<dyn Overview>) {
        println!("{}", item.overview());
    }

    let obj = Box::new(Obj{});
    call_obj_box(obj);
    // println!("{}",obj.overview()); // move


    // 定义明治牛奶
    trait MeijiMilk {
        fn price(&self) -> f64;
    }

    // 定义abc超市
    struct AbcMarket {}
    impl MeijiMilk for AbcMarket {
        fn price(&self) -> f64 {
            10.0
        }
    }

    // 定义coco超市
    struct CocoMarket {}
    impl MeijiMilk for CocoMarket {
        fn price(&self) -> f64 {
            20.0
        }
    }
    fn get_price(item: impl MeijiMilk) {
        println!("{}", item.price());
    }

    get_price(AbcMarket{});
    get_price(CocoMarket{});

    fn get_markets_price(markets: Vec<Box<dyn MeijiMilk>>) {
        for market in markets {
            println!("{}", market.price());
        }
    }

    let markets:Vec<Box<dyn MeijiMilk>> = vec![Box::new(AbcMarket{}), Box::new(CocoMarket{})];
    get_markets_price(markets);
}

fn trait_obj_pro() {
    // fn call(item1: &impl Trait, item2: &impl Trait)  可以是不同类型
    // fn call_generic<T: Trait>(item1: &T, item2: &T)  必须是相同类型

    // 如果实现了多个 Trait
    // fn call(item1: &(impl Trait + Trait2))
    // fn call_generic<T: Trait + Trait2>(item1: &T)

    trait Overview {
        fn overview(&self) -> String {
            String::from("trait Overview")
        }
    }

    trait Overview2 {
        fn hello_world(&self) -> String {
            String::from("hello world")
        }
    }

    struct Course{
        headline: String,
        author: String,
    }

    impl Overview for Course {
        fn overview(&self) -> String {
            format!("{} from {}", self.headline, self.author)
        }
    }

    impl Overview2 for Course {

    }

    let course = Course {
        headline: "Rust".to_owned(),
        author: "rust".to_owned(),
    };

    println!("{}", course.overview());
    println!("{}", course.hello_world());

    fn call<T: Overview + Overview2>(item1: &T) {
        println!("{}", item1.overview());
        println!("{}", item1.hello_world());
    }

    call(&course);
}