
/// One important part of this code is that the `Messenger` trait
/// has one method called `send` that takes an immutable reference
/// to `self` and the text of the message. 
/// This is the interface our mock objects need to have.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    /// The behavior of `set_value`
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_message: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //self.sent_messages.push(String::from(message))
            self.sent_messages.borrow_mut() // return RefMut<Vec<String>>
                .push(String::from(message))
        }
    }
    
    #[test]
    fn warning_message() {
        // Create a new `MockMessenger` with an empty list of messages
        let mock_messenger = MockMessenger::new();

        // Create a new `LimitTracker`, giving it a reference to the new
        // `MockMessenger` and a max value of 100
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // Set the value to greater than 75%
        limit_tracker.set_value(80);

        /* (In the commented out assert_eq!() statement)
         *
         * Since the `send` method takes an immutable reference to `self`,
         * we can't modify the `MockMessenger` to keep track of the messages.
         * Moreover, we can't take the suggestion from the error text to use
         * `&mut self` because the signature of `send` wouldn't match the
         * signature in the `Messenger` trait.
         * */
        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        
        // For the above reasons, we must use "interior mutability". 
        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);

    }
}
