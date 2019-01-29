extern crate tokio;
// extern crate futures;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;


// Bind a TcpListener to a local port.
// Define a task that accepts inbound connections and processes them.
// Spawn the server task.
// Start the Tokio runtime



fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // process each inbound client connection 
    let server = listener.incoming()
    .map_err(|e| println!("failed to accept socket; error = {:?}", e))
    .for_each(move |socket| {
    	// split socket stream into its reading and writing components
    	let (reader, writer) = socket.split();

    	// println!("{:?}", reader);

    	// read data from the reader stream and write to the writer stream
    	let num_bytes = io::copy(reader, writer);

    	// match over the Result from the Future data
    	let msg = num_bytes.then(move |result| {
    		println!("{:?}", result);
    		match result {
    			Ok((num_bytes, _, _)) => {
    				println!("Wrote {} bytes", num_bytes);
    			},
    			Err(e) => println!("Error: {}", e),
    		}
    		
    		Ok(())
    	});

    	tokio::spawn(msg);
    	Ok(())
    });

    println!("Server is running on localhost:6142");

    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);
    Ok(())
}









