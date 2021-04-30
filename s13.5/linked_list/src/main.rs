/**
## 预备知识
1. Option
2. Box
3. take / replace / swap `std::mem`
   take: take 用于 Option，可以用 None 把 Option 里的内容转移出来，取得其所有权。take 改变了数据，
   因此被执行的 Option 对象必须可变，take 之后，此 Option 等于 None。
   replace: 使用的范围更广，很多数据结构都支持此方法，可以自己构造一个语义上的空对象来换取数据。
   swap: 交换两个值 let mut x = 5; let mut y = 6; mem::swap(&mut x, &mut y)

3. 模式解构
if let Some(ref mut _ret) = ret {}
if let Some(ref _ret) = ret {}
if let Some(mut _ret) = ret {}
**/

fn main() {
    println!("Hello, world!");
    let mut link = LinkNode::new(0);
    for i in 1..10 {
        link.append(i);
    }

    println!("link: {:?}", link);
    let r_link = LinkNode::reverse_list(Some(Box::new(link)));
    println!("r_link: {:?}", r_link);

    // println!("middle node: {:?}", LinkNode::middle_node(r_link));
}

#[derive(Debug, Clone)]
struct LinkNode {
    val: Option<i32>,
    next: Option<Box<LinkNode>>,
}

impl LinkNode {
    fn new(val: i32) -> Self {
        LinkNode {
            val: Some(val),
            next: None,
        }
    }

    // 返回最底层节点
    fn get_last_mut(&mut self) -> &mut Self {
        match self.next {
            Some(ref mut box_node) => box_node.get_last_mut(),
            None => self,
        }
    }

    fn append(&mut self, val: i32) {
        let node = LinkNode::new(val);
        self.get_last_mut().next = Some(Box::new(node));
    }

    // fn pop(&mut self) -> Option<i32> {
    //     if let Some(r) = self.val {
    //         if let Some(ref rc) = self.next {
    //             replace(&mut self, *rc)
    //         }
    //
    //         return Some(r);
    //     }
    //
    //     None
    // }
}


impl LinkNode {
    // 反转节点
    fn reverse_list(head: Option<Box<LinkNode>>) -> Option<Box<LinkNode>> {
        let mut prev = None; // 上一个节点
        let mut cur = head; // 当前节点
        // println!("now cur: {:?}", cur);
        while let Some(mut node) = cur {
            cur = node.next.take(); // 取出
            // println!("next cur: {:?}", cur);
            // println!("prev cur: {:?}", prev);
            // println!("node cur: {:?}", node);
            node.next = prev;
            // println!("node2 cur: {:?}", node);
            // println!();
            prev = Some(node);
        }

        prev
    }
}

impl LinkNode {
    fn len(&mut self) -> usize {
        let mut total = 0;
        let mut r = self;
        while let Some(ref mut item) = r.next {
            total += 1;
            r = item
        }

        total
    }

    // fn get(&mut self, idx: usize) -> Option<Box<LinkNode>> {
    //     let mut step = 0;
    //     if let Some(ref mut r) = self.next {
    //         let mut rc = r;
    //
    //         while step < idx {
    //             step += 1;
    //             if let Some(ref mut cc) = r.next {
    //                 rc = cc
    //             }
    //             if step == idx {
    //                 if let Some(ref mut rrc) = rc.next {
    //
    //                 }
    //             }
    //         }
    //     }
    //
    //     None
    // }

    // fn middle_node(head: Box<LinkNode>) -> Box<LinkNode> {
    //     let mut head = head;
    //     let mut total = head.len(); // 获取总长度
    //
    //     if total % 2 == 0 {
    //         total = total / 2
    //     } else {
    //         total = (total - 1) / 2
    //     }
    //
    //     head
    // }

    // fn middle_node(head: Option<Box<LinkNode>>) -> Option<Box<LinkNode>> {
    //     let mut head = head;
    //     let mut total = head.clone().unwrap().len(); // 获取总长度
    //
    //     if total %2 == 0 {
    //         total = total / 2
    //     }else {
    //         total = (total-1)/2
    //     }
    //
    //     let mut step = 0; // 中间位置
    //     while step < total {
    //         step += 1;
    //         if let Some(_head) = head {
    //             head = _head.next;
    //         }
    //     }
    //
    //     head
    // }
}
