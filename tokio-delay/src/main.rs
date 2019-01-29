use tokio::prelude::*;
use tokio::timer::Delay;


use std::time::{Duration, Instant};

fn main() {
	let when = Instant::now() + Duration::from_millis(5000);
	let task = Delay::new(when)
	    .and_then(|_| {
	    	println!("Hello World");
	    	Ok(())
	    })
	    .map_err(|e| panic!("delay errored; err={:?}", e));

	tokio::run(task);    
}
