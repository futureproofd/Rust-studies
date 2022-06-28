// Interior Mutability: RefCell<T>

/*
A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably.
For example, this code won’t compile:

    let x = 5; <-- x isn't declared as mutable
    let y = &mut x;
*/

/*
However, there are situations in which it would be useful for a value to mutate itself in its methods but
 appear immutable to other code. Code outside the value’s methods would not be able to mutate the value.
  Using RefCell<T> is one way to get the ability to have interior mutability.
*/

// Example: - a mock object - or the Struct equivalent

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
            // simply sends a message according to our implementation details.
            // in order to test this (below), we will need a mock object (Struct) to hold the messages we expect
            self.messenger.send("Error: You're over your quota.")
        } else if percentage_of_max >= 9.0 {
            self.messenger.send("You've used up 90% of your quota.")
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("You've used up 75% of your quota.")
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
        fn send(&self, msg: &str) {
            //We can’t modify the MockMessenger to keep track of the messages,
            //because the send method takes an immutable reference to self.
            // self.sent_messages.push(String::from(msg))

            // instead, get a mutable reference (to the vector inside)
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
    }

    #[test]
    fn sends_75_percent_warning_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
