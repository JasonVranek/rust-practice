// A library that keeps track of the number of messages sent by a user
// and warns when they are reaching their limit.


pub trait Messenger {
	fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
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

    	pub fn set_value(&mut self, value: usize) {
    		self.value = value;

    		let percentage_of_max = self.value as f64 / self.max as f64;

    		if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
    			self.messenger.send("Warning: you have used up over 75% of your
    				quota!");
    		} else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
    			self.messenger.send("Warning: you have used up over 90% of your
    				quota!");
    		} else if percentage_of_max >= 1.0 {
    			self.messenger.send("Error: you are over your quota!");
    		}

    	}
    }

#[cfg(test)]
mod test {
	use super::*;
	use std::cell::RefCell;

	struct MockMessenger {
		sent_messages: RefCell<Vec<String>>,
	}

	impl MockMessenger {
		fn new() -> MockMessenger {
			MockMessenger { sent_messages: RefCell::new(vec![]) }
		}
	}

	impl Messenger for MockMessenger {
		fn send(&self, message: &str) {
			// send takes &self to mirror the signature of the Messenger trait definition.
			// This gives an error that interior mutability can fix. Self is still an immutable
			// reference which fits the definition, but we are allowed to call the borrow_mut()
			// method because our sent_messages field is wrapped with RefCell.
			self.sent_messages.borrow_mut().push(String::from(message));

			// Since RefCell borrowing is enforced at runtime, uncommenting the below
			// will cause the program to panic because there are 2 mutable borrows which
			// is not allowed:
			// let mut_borrow_1 = self.sent_messages.borrow_mut();
			// let mut_borrow_2 = self.sent_messages.borrow_mut();

		}
	}

	#[test]
	fn it_send_an_over_75_percent_warning_message() {
		// Create a MockMessanger
		let mock_messenger = MockMessenger::new();

		let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

		limit_tracker.set_value(76);

		assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

	}
}






















