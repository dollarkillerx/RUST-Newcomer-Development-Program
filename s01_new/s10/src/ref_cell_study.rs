use std::cell::RefCell;
use std::rc::Rc;
pub enum Link<T> {
    Nil,
    Cons(Rc<RefCell<T>>, Rc<Link<T>>)
}



pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }else if percentage_of_max >= 0.9 {
            self.messenger.send("You've used up over 90% of your quota!");
        }else if percentage_of_max >= 0.75 {
            self.messenger.send("You're using up over 75% of your quota!");
        }else if percentage_of_max >= 0.5 {
            self.messenger.send("You're using up over 50% of your quota!");
        }else if percentage_of_max >= 0.25 {
            self.messenger.send("You're using up over 25% of your quota!");
        }else if percentage_of_max >= 0.0 {
            self.messenger.send("You're using up over 0% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(message.to_owned());
            self.sent_messages.borrow_mut().push(message.to_owned());
        }
    }


    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }


}