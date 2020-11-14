struct ListNode<T> {
    data: T,
    n: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T, node: Option<Box<ListNode<T>>>) -> ListNode<T> {
        ListNode { data, n: node }
    }
}

struct List<T> {
    n: Option<Box<ListNode<T>>>,
    total: i32,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { total: 0, n: None }
    }

    fn append(&mut self, data: T) {
        self.total += 1;
        let o = self.n.take();
        let n = Box::new(ListNode::new(data, o));
        self.n = Some(n);
    }

    fn pop(&mut self) -> Option<T> {
        let p = self.n.take();
        match p {
            None => None,
            Some(t) => {
                self.total -= 1;
                let o = t.data;
                self.n = t.n;
                Some(o)
            }
        }
    }
}

fn main() {
    let mut l = List::new();
    for i in 0..100 {
        l.append(i);
    }

    for _ in 0..101 {
        println!("{:?}", l.pop());
    }
}
