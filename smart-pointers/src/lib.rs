use std::cell::RefCell;

// ------------------------------------------------------------------
// Intrior Mutablility
// In rust we can not borrow imutable varialbe as mutable borrow like
// let a = 3;
// let b = &mut a;
// The reust rulles inforces that we can not have two imputable references pointing to the same
// variable at the same time.
// `RefCell` applies this rule in runtime not in complie time
// `RefCell` is not allocating space in Heap like RC or Box but is is desgined to apply rust borrow
// rules in runtime not in compile time.
// to allocate a place in heap while applying rust rules in runtime we might do
// let a = RefCell::new(Box::new(3));
// `RefCell` has manily two methods: `borrow()` and `borrow_mute()`
// `RefCell` is not working in multithreading but `Mutex` works
// ------------------------------------------------------------------
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
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
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
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
            // NOTE: we can not mutate a variable is it was inputed as imutbalbe. But we have to
            // make all methods and functino using our trait as mut
            self.sent_messages.borrow_mut().push(String::from(message));

            // BUG: Creating runtime error
            // let mut a = self.sent_messages.borrow_mut();
            // let mut b = self.sent_messages.borrow_mut();
            // a.push("hello".to_string());
            // b.push("hello".to_string());
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
