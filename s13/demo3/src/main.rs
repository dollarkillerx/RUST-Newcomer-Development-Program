use std::fmt::Debug;

trait DoSomething<T> {
    fn do_sth(&self, value: T);
}

impl <'a,T:Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value:T) {
        println!("{:?}",value);
    }
}

//  高阶生命周期, 高阶限定
fn foo<'a> (b: Box<for<'f> DoSomething<&'f usize>>) {
    let s: usize = 10;
    b.do_sth(&s);
}

fn main() {
    println!("Hello, world!");

    let x = Box::new(&2usize);
    foo(x);
}
