use std::mem;
use std::option::Option::Some;

fn main() {
    println!("Hello, world!");
    let mut r = LinkList::new();
    for i in 0..10 {
        r.push(i);
    }

    println!("r: {:?}", r);
    while let Some(r) = r.pop() {
        println!("r 1 : {:?}", r);
    }
}

#[derive(Debug)]
struct LinkNode {
    val: usize,
    next: Option<Box<LinkNode>>,
}

impl LinkNode {
    fn new(val: usize) -> Self {
        LinkNode {
            val,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkList {
    len: usize,
    link: Option<Box<LinkNode>>,
}


impl LinkList {
    fn new() -> Self {
        LinkList {
            len: 0,
            link: None,
        }
    }

    fn push(&mut self, elem: usize) {
        match self.link {
            None => {
                self.len += 1;
                self.link = Some(Box::new(LinkNode::new(elem)))
            }
            Some(_) => {
                self.len += 1;

                let new_node = Box::new(LinkNode {
                    val: elem,
                    next: mem::replace(&mut self.link, None),
                });
                self.link = Some(new_node)
            }
        }
    }

    fn pop(&mut self) -> Option<usize> {
        match mem::replace(&mut self.link, None) {
            None => None,
            Some(r) => {
                let rc = r.next;
                let val = r.val;
                self.link = rc;
                Some(val)
            }
        }
    }
}

impl Drop for LinkList {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.link, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
            // boxed_node 退出时自动drop
        }
    }
}