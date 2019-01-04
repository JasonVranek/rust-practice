extern crate tokio;

// Asynchronous networking modules similar to the std library
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;


fn main() {
    // Parse the address of whatever server we're talking to
    let addr = "127.0.0.1:6142".parse().unwrap();

    let client = TcpStream::connect(&addr).and_then(|stream| {
        println!("Created the stream! (it is a Future of the created TcpStream)");

	    io::write_all(stream, "hello world\n").then(|result| {
	      println!("wrote to stream; success={:?}", result.is_ok());
	      Ok(())
	    })
	}).map_err(|err| {
    	println!("All tasks have an empty Error() type: {}", err);
    });

    println!("About to create the stream and write to it...");
	tokio::run(client);
	println!("Stream has been created and written to.");

}


// In a separate terminal listen for tcp using:  nc -l 6142
