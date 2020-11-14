#[derive(Debug)]
struct StackNode<T> {
    data: T,
    next: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { top: None }
    }

    fn push(&mut self, data: T) {
        let mut node = StackNode { data, next: None };
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let node = self.top.take();
        match node {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.data)
            }
        }
    }
}


fn main() {
    let mut l = Stack::new();
    for i in 1..100 {
        l.push(i);
    }

    println!("{}", l.pop().unwrap());
}





































