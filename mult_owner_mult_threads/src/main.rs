// The atomic reference count (Arc) type allows us to have
// data shared across threads
use std::sync::{Mutex, Arc};
use std::thread;



fn main() {
	// Create a mutex that is safe to be shared across threads
	// A mutex provides interior mutability
    let counter = Arc::new(Mutex::new(0));
    // A vector to store the thread handles for joining later
    let mut handles = vec![];

    // spawn 10 threads, each one incrementing the counter mutex in their threads
    for _ in 0..10 {
    	// Clone a reference to the mutex that is threadsafe
    	let counter = Arc::clone(&counter);

    	// Create a thread, moving this counter reference into the closure
    	// to be executed in the thread.
    	let handle = thread::spawn(move || {
    		// Thread will block until it acquires a lock, and will panic
    		// if there is an error
    		let mut num = counter.lock().unwrap();

    		// Dereference the mutex and modify it
    		*num += 1;
    	});
    	// Safe the threads handle
    	handles.push(handle);
    }

    // Block until threads are finished using join()
    for handle in handles {
    	handle.join().unwrap();
    }

    // Aquire Mutex lock, unwrap Option type, and dereference  
    println!("Result: {}", *counter.lock().unwrap());
}
