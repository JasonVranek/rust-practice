use tokio::prelude::*;
use tokio::timer::Interval;
use tokio::io;
use std::time::{Duration, Instant};

fn main() {
	// The first param of Interval is when the interval starts, the second is how long
	// between intervals.
	// let task = Interval::new(Instant::now(), Duration::from_millis(100))
	//     .take(10)
	//     .for_each(|instant| {
	//     	println!("fire, instant={:?}", instant);
	//     	Ok(())
	//     })
	//     .map_err(|e| panic!("interval errored={:?}", e));

	let task = interval_task();

	tokio::run(task);
}

fn interval_task() ->  Box<Future<Item = (), Error = ()> + Send>{
	let task = Interval::new(Instant::now(), Duration::from_millis(500))
	    // .take(10)
	    .for_each(|instant| {
	    	println!("fire, instant={:?}", instant);
	    	Ok(())
	    })
	    .map_err(|e| panic!("interval errored={:?}", e));
	Box::new(task)
}
