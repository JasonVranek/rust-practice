// This example uses tokio streams and futures. The purpose
// is to demonstrate how to write a program that can send data
// that is created asnychronously over a connection (stream), to
// be processed as future objects.
#[macro_use] // for try_ready!
extern crate tokio;
extern crate futures;

// A stream that yields () values at the requested time interval,
// Calling Interval::poll between intervals results in Async::NotReady being returned.
use tokio::timer::Interval;

use futures::{Future, Stream, Poll, Async, try_ready};
use std::fmt;

// For passing into Interval timer
use std::time::Duration;

pub struct Fibonacci {
	interval: Interval,		
	curr: u64,
	next: u64,
}

impl Fibonacci {
	fn new(duration: Duration) -> Fibonacci {
		Fibonacci {
			// The interval is to simulate an expensive calculation
			interval: Interval::new_interval(duration),
			curr: 1,
			next: 1,
		}
	}
}

impl Stream for Fibonacci {
	type Item = u64;

	// Streams don't yield errors
	type Error = ();

	fn poll(&mut self) -> Poll<Option<u64>, ()> {
		// Wait until the next interval to simulate calulation
		try_ready!(
			self.interval.poll()
			// The interval can fail if the Tokio runtime is unavailable.
            // In this example, the error is ignored.
            .map_err(|e| (println!("There was an error: {}", e)))
		);

		let curr = self.curr;
		let next = curr + self.next;

		self.curr = self.next;
		self.next = next;

		// Done modifying state, return Option<state type> wrapped with Async::Ready
		// to satify the poll method that must be implemente on Stream types.
		Ok(Async::Ready(Some(curr)))
	}
}


// Now is an implementation of an object that implements the Future trait.
// All it does is display the first 10 items in the Fibonacci stream.
pub struct DisplayFib<T> {
	stream: T,
	curr: usize,
}

impl<T> DisplayFib<T> {
	fn new(stream: T) -> DisplayFib<T> {
		DisplayFib {
			stream,
			curr: 0,
		}
	}
}

impl<T> Future for DisplayFib<T> 
where 
    T: Stream,
    T::Item: fmt::Display,		// For printing the values
{
	type Item = ();
	type Error = T::Error;

	fn poll(&mut self) -> Poll<(), Self::Error> {
		while self.curr < 50 {
			// try_ready! is a macro for extracting the successful type of a Poll<T, E>.
			let value = match try_ready!(self.stream.poll()) {
				Some(value) => value,
				None => break,
			};

			println!("value #{} = {}", self.curr, value);
			self.curr += 1;
		}
		
		Ok(Async::Ready(()))
	}
}




fn main() {
	synchronous_using_combinators(50);
	
	asynchronous_stream();

}

fn asynchronous_stream() {
	// Create the fiboncacci stream
	let fib = Fibonacci::new(Duration::from_millis(1));

	// Create the display to consume the stream
	let display = DisplayFib::new(fib);

	tokio::run(display);
}


// The take combinator limits the fibonacci stream to 10 values. 
// The for_each combinator asynchronously iterates the stream values. 
// for_each consumes the stream and returns a future that completes 
// once the closure was called once for each stream value. It is 
// the asynchronous equivalent to a rust for loop.
fn synchronous_using_combinators(count: u64) {
	tokio::run(
		recursive_fib_stream().take(count)
		    .for_each(|num| {
		    	println!("{}", num);
		    	Ok(())
		    })
    );


}

use futures::stream;
fn recursive_fib_stream() -> impl Stream<Item = u64, Error = ()> {
	stream::unfold((1, 1), |(curr, next)| {
		let new_next = curr + next;

		Some(Ok((curr, (next, new_next))))
	})
}









