#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    println!("Hello, world!");
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(16,Box::new(List::Nil))));
    println!("list: {:?}", list);
}

enum List2{
    Empty,
    Elem(i32, Box<List2>)
}