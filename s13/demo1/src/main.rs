trait Trait {
    fn f(self);
}

impl<T> Trait for fn(T) {
    fn f(self) {
        println!("1");
    }
}

impl<T> Trait for fn(&T) {
    fn f(self) {
        println!("2");
    }
}

fn main() {
    println!("Hello, world!");

    let a: fn(_) = |_: u8| {};
    let b: fn(_) = |_: &u8| {};
    let c: fn(&_) = |_: &u8| {};

    a.f();
    b.f();
    c.f();
}
