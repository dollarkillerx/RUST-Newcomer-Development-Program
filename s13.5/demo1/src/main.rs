#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    println!("Hello, world!");
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(16, Box::new(List::Nil))));
    println!("list: {:?}", list);
}

enum List2 {
    Empty,
    Elem(i32, Box<List2>),
}

struct CaCher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> CaCher<T>
    where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CaCher<T> {
        CaCher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

