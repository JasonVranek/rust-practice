use std::thread;
use std::sync::mpsc;

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let val = format!("'Hello'");
		println!("Sending {}, over thread through a channel", val);

		tx.send(val).unwrap();
	});

	// recv() blocks until a value is sent over the channel
	// The channel returns a Result<T, E>
	let received = rx.recv().unwrap();
	println!("Received {} from a separate thread", received);

	// The main thread will panic since the channel closes after
	// the message has been sent.
	let received = rx.recv().unwrap();
	println!("Received {} from a separate thread", received);
}
