use std::mem;
use std::fmt::Debug;

pub struct List<T>
    where T: Debug
{
    head: Link<T>,
}

// 此处不强制T类型
type Link<T> = Option<Box<Node<T>>>;

struct Node<T>
    where T: Debug
{
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
    where T: Debug
{
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // match self.head.take() {
        //     None => None,
        //     Some(r) => {
        //         self.head = r.next;
        //         Some(r.elem)
        //     }
        // }
    }
}

impl<T> Drop for List<T>
    where T: Debug
{
    fn drop(&mut self) {
        let mut rec = mem::replace(&mut self.head, None);
        while let Some(mut r) = rec {
            rec = r.next.take();
        }
    }
}