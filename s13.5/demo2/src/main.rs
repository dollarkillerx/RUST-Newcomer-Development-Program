use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}

// struct Link {
//     head: Link
// }

#[derive(Debug)]
enum Link<T>
    where T: Debug
{
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T>
    where T: Debug
{
    elem: T,
    next: Link<T>,
}

impl <T> Link<T>
    where T: Debug
{
    

}