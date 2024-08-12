use std::ops::Add;
use std::time::Instant;

fn main() {
    // 本章主要： 重载操作符operator, 多态, 常见的trait, 迭代器
    demo1();
    println!("{}", "-".repeat(30));
    demo2(); // 多态
    println!("{}", "-".repeat(30));
    demo3(); // Debug, Clone, Copy,
    println!("{}", "-".repeat(30));
    demo4(); // 迭代器 重要
}

fn demo1() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T
    }

    // where T: Add<Output = T> 约束T必须实现Add
    impl<T> Add for Point<T> where T: Add<Output = T> {
        type Output = Point<T>;
        fn add(self, other: Point<T>) -> Point<T> {
            Point {
                x: self.x + other.x,
                y: self.y + other.y
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

fn demo2() {
    trait Driver {
        fn drive(&self);
    }

    struct Car;
    impl Driver for Car {
        fn drive(&self) {
            println!("Car is driving");
        }
    }

    struct SUV;
    impl Driver for SUV {
        fn drive(&self) {
            println!("SUV is driving");
        }
    }

    fn road(vehicle:&dyn Driver) {
        vehicle.drive();
    }

    road(&Car {});
    road(&SUV {});
}

fn demo3() {

}

fn demo4() {
    fn sum(arr: &[i32]) -> i32 {
        arr.iter().sum()
    }

    fn sum_loop(arr: &[i32]) -> i32 {
        let mut sum = 0;
        for i in arr {
            sum += i;
        }
        sum
    }

    const ARRAY_SIZE: usize = 1_000_0;
    let array:Vec<_> = (1..=ARRAY_SIZE as i32).collect();

    let start = Instant::now(); // 开始计时
    println!("{}", sum(&array));
    let duration = start.elapsed(); // 计算运行时间
    println!("iter Duration: {:?}", duration); // 370.9us

    let start = Instant::now(); // 开始计时
    println!("{}", sum_loop(&array));
    let duration = start.elapsed(); // 计算运行时间
    println!("loop Duration: {:?}", duration); // 336.1us
}