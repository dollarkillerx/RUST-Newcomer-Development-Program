use crate::List::Cons;

fn main() {
    enum1();
    list1();
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

impl WebEvent {
    fn inspect(&self) {
        match self {
            WebEvent::PageLoad => {
                println!("Web Event::PageLoad");
            }
            WebEvent::PageUnload => {
                println!("WebEvent::PageUnload");
            }
            WebEvent::KeyPress(s) => {
                println!("WebEvent::KeyPress({})", s);
            }
            WebEvent::Click { x, y } => {
                println!("WebEvent::Click{{ {}, {}}}", x, y);
            }
        }
    }
}

type Wb = WebEvent;

fn enum1() {
    let a = WebEvent::PageLoad;
    let b = WebEvent::PageUnload;
    let c = WebEvent::KeyPress('a');
    let d = WebEvent::Click { x: 12, y: 23 };

    a.inspect();
    b.inspect();
    c.inspect();
    d.inspect();

    let e = Wb::PageLoad;
    e.inspect();
}


enum ListError {
    Nil,
    Msg(String),
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    fn append(self, elem: T) -> List<T> {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> i32 {
        let mut output = 0;
        let mut i = self;
        loop {
            match i {
                List::Cons(_, t) => {
                    output += 1;
                    i = t;
                    continue;
                }
                List::Nil => {
                    return output;
                }
            }
        }
    }

}

fn list1() {
    let mut l: List<i32> = List::new();
    println!("len: {}", l.len());

    l = l.append(12);
    l = l.append(12);
    l = l.append(12);
    l = l.append(12);
    l = l.append(12);

    println!("len: {}", l.len());
}