extern crate tokio;
#[macro_use] 
extern crate futures;
extern crate bytes;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use futures::sync::mpsc;
use futures::future::{self, Either};
use bytes::{BytesMut, Bytes, BufMut};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};


// Abbreviations for the transmit and receive parts of the message channel.
type Tx = mpsc::UnboundedSender<Bytes>;
type Rx = mpsc::UnboundedReceiver<Bytes>;


// Hashmap to store the socket addresses of all the peers in the chat
struct Shared {
	peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
	fn new() -> Self {
		Shared {
			peers: HashMap::new(),
		}
		
	}
}


struct Lines {
	socket: TcpStream,
	rd: BytesMut,
	wr: BytesMut,
}

impl Lines {
	// Create a new Lines codec backed by the socket. A codec is a
	// a type that takes a byte stream type (AsyncRead + AsyncWrite) and exposes a read
	// and write API at the frame level.
	fn new(socket: TcpStream) -> Self {
		Lines {
			socket,
			rd: BytesMut::new(),
			wr: BytesMut::new(),
		}
	}
}

impl Stream for Lines {
	type Item = BytesMut;
	type Error = io::Error;

	fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
		// Read any new data that might have been received from socket.

		// Track if the socket has been closed
		let socket_closed = self.fill_read_buf()?.is_ready();

		// Look for the escape characters that mean the end of a chat message: \r\n
		// pos will be an Option type
		let pos = self.rd.windows(2)
		    .position(|bytes| bytes == b"\r\n");

		if let Some(pos) = pos {
			// remove the line from the read buffer and set it to 'line'
			let mut line = self.rd.split_to(pos + 2);

			// Remove the trailing \r\n
			line.split_off(pos);

			return Ok(Async::Ready(Some(line)));
		}

		if socket_closed {
			Ok(Async::Ready(None))
		} else {
			// Only ever return NotReady unless the function received an Async::NotReady
			// itself which this did from the fill_read_buf function.
			Ok(Async::NotReady)
		}
	}
}

impl Lines {
	fn fill_read_buf(&mut self) -> Result<Async<()>, io::Error> {
		loop {
			// ensure that the read buffer has a capacity
			self.rd.reserve(1024);

			// read the data into the buffer
			let n = try_ready!(self.socket.read_buf(&mut self.rd));

			if n == 0 {
				return Ok(Async::Ready(()));
			}
		}
	}

	fn buffer(&mut self, line: &[u8]) {
		// push the line onto the end of the write buffer using the put function from BufMut trait
		self.wr.put(line);
	}

	// poll_flush only returns Ready once all the queued data has been successfully written to socket
	fn poll_flush(&mut self) -> Poll<(), io::Error> {
		// Write all the data from the buffer
		while !self.wr.is_empty() {
			// Try to write some bytes to the socket
			let n = try_ready!(self.socket.poll_write(&self.wr));

			// A successful write on a non empty buffer should always yield > 0 bytes
			assert!(n > 0);

			// discard the first 'n' bytes of the buffer (since we already wrote them)
			let _ = self.wr.split_to(n);
		}

		Ok(Async::Ready(()))
	}
}


struct Peer {
	// name of the peer. This is the first line received from the client.
	name: BytesMut,
	
	// TCP socket wrapped in the Lines codec
	lines: Lines,

	// handle to the shared chat state
	state: Arc<Mutex<Shared>>,

	// Receive half of the message channel. This is used to receive messages from peers.
	// When a message is received off of this Rx, it will be written to the socket.
	rx: Rx,

	// Client socket address. Used as the key in the peers HashMap. The address is saved
	// so the Peer drop implementation can clean up its entry.
	addr: SocketAddr,
}

impl Peer {
	fn new(name: BytesMut,
		   state: Arc<Mutex<Shared>>,
		   lines: Lines) -> Peer 
	{
		// Get the client socket address
		let addr = lines.socket.peer_addr().unwrap();

		// Create a channel for this peer
		let (tx, rx) = mpsc::unbounded();

		// add an entry for this Peer in the shared state map
		state.lock().unwrap()
		    .peers.insert(addr, tx);

		Peer {
			name,
			lines, 
			state,
			rx,
			addr,
		}
	}
}

impl Drop for Peer {
	fn drop(&mut self) {
		self.state.lock().unwrap().peers
		    .remove(&self.addr);
	}
}

impl Future for Peer {
	type Item = ();
	type Error = io::Error;

	fn poll(&mut self) -> Poll<(), io::Error> {
		// Receive all messages from peers
		loop {
			// polling an UnboundedReceiver cannot fail so safe to unwrap
			match self.rx.poll().unwrap() {
				Async::Ready(Some(v)) => {
					// buffer the line, once all lines are buffered they will be flushed to the socket
					self.lines.buffer(&v);
				}
				_ => break,
			}
		}

		// flush the write buffer to the socket
		let _ = self.lines.poll_flush()?;

		// Read new lines from the socket
		while let Async::Ready(line) = self.lines.poll()? {
			println!("Received line ({:?} : {:?}", self.name, line);

			if let Some(message) = line {
				// append the peers name to the front of the line:
				let mut line = self.name.clone();
				line.put(": ");
				line.put(&message);
				line.put("\r\n");

				// freeze the data from mutable to immutable to allow for zero copy cloning
				let line = line.freeze();

				// send the line to all other peers
				for (addr, tx) in &self.state.lock().unwrap().peers {
					// don't send to self
					if *addr != self.addr {
						tx.unbounded_send(line.clone()).unwrap();
					}
				}
			} else {
				// EOF reached, remote client has disconnected, nothing left to do
				return Ok(Async::Ready(()));
			}
		}

		Ok(Async::NotReady)

	}
}

fn process(socket: TcpStream, state: Arc<Mutex<Shared>>) {
	// wrap the socket with the Lines codec
	let lines = Lines::new(socket);

	// into_future combinator extracts the first item from the lines stream. into_future
	// takes a Stream and converts it into a future of (first, rest) where rest is the 
	// original stream instance.
	let connection = lines.into_future()
		// must map the error to get into_future to have the right error type
	    .map_err(|(e, _) | e)
	    // process the first received line as the client's name
	    .and_then(|(name, lines)| {
	    	let name = match name {
	    		Some(name) => name,
	    		None => {
	    			// TODO: handle a client that disconnects early.
	    			return Either::A(future::ok(()));
	    		}
	    	};

	    	println!("{:?} is joining the chat", name);
	    
		    // create the peer
		    let peer = Peer::new(
		    	name,
		    	state,
		    	lines);

		    // Wrap peer with Either::B to make the return type fit
		    Either::B(peer)
	    })
	    .map_err(|e| {
	    	println!("Connection error = {:?}", e);
	    });

	    
	// Spawn the task to the current Tokio runtime
	tokio::spawn(connection);
}

fn main() -> Result<(), Box<std::error::Error>> {
	// Create an instance of the shared state to be moved into the task that accepts incoming connections
	let state = Arc::new(Mutex::new(Shared::new()));

	// Bind a TcpListener to a local port
	let addr = "127.0.0.1:6142".parse().unwrap();
	let listener = TcpListener::bind(&addr).unwrap();

	let server = listener.incoming().for_each(move |socket| {
		process(socket, state.clone());

		Ok(())
	})
	.map_err(|err| {
		// Better error handling in future...
		println!("accept error = {:?}", err);
	});

	println!("running the server on localhost:6142");

	// Starting the server starts the Tokio runtime -> reactor, threadpool, executor, etc.
	// Blocks thread until all the spawned tasks have completed.
	tokio::run(server);
	Ok(())
}












