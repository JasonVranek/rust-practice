use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
	// Initialize the channel
    let (tx, rx) = mpsc::channel();

    // clone reference to transmitter to allow
    // multiple threads to send values to the same receiver

    let mut tx_channels = Vec::new();
    for _ in 0..10 {
    	tx_channels.push(mpsc::Sender::clone(&tx));
    }

    // create 10 threads
    for i in 0..10 {
    	let tx = tx_channels.pop().unwrap();
    	thread::spawn(move || {
    		tx.send(i).unwrap();
    		thread::sleep(Duration::from_secs(1));
    	});
    }

    loop {
    	match rx.try_recv() {
    		Ok(msg) => println!("Got: {}", msg),
    		Err(e) => {
    			println!("Done Receiving: {}", e);
    			break;
    		}
    	}
    }
}
