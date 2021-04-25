use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
    test_a();
}

// struct Link {
//     head: Link
// }

// #[derive(Debug)]
// enum Link<T>
//     where T: Debug
// {
//     Empty,
//     More(Box<Node<T>>),
// }
//
// #[derive(Debug)]
// struct Node<T>
//     where T: Debug
// {
//     elem: T,
//     next: Link<T>,
// }
//
// impl <T> Link<T>
//     where T: Debug
// {
//     fn new() -> Link<T> {
//         Link::Empty
//     }
//
//     fn append(&mut self, elem: T) {
//         self
//     }
// }

#[derive(Debug)]
struct Link<T>
    where T: Debug
{
    node: Option<Box<LinkNode<T>>>
}

#[derive(Debug)]
struct LinkNode<T>
    where T: Debug
{
    node: T,
    next: Option<Box<LinkNode<T>>>,
}

impl<T> Link<T>
    where T: Debug
{
    fn new() -> Link<T> {
        return Link {
            node: None
        };
    }

    fn append(&mut self, elem: T) {
        let mut ps = self.node.as_mut();
        if let None = ps {
            self.node = Some(Box::new(LinkNode { node: elem, next: None }));
            return;
        }

        let  mut af:Option<&mut Box<LinkNode<T>>> = None;
        loop {
            if let Some(p) = ps {
                af = Option::from(p);
                ps = p.next.as_mut();
            } else {
                af.unwrap().next = Some(Box::new(LinkNode { node: elem, next: None }));
                break;
            }
        }
    }
}

fn test_a() {
    let mut c = Link::new();
    for i in 1..10 {
        c.append(i);
    }

    println!("r : {:?}", c);
}