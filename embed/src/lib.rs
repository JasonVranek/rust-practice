use std::thread;

#[no_mangle]
pub extern fn process(msg: u128) {
    // println!("Received message: {}", msg);
    let handles: Vec<_> = (0..10).map(|_| {
        let count: u128 = msg;
        thread::spawn(move || {
            let mut x = 0;
            
            for _ in 0..count {
                x += 1
            }
        x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
    println!("done!");
}