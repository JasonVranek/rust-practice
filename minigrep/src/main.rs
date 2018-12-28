extern crate minigrep;  // Import the lib.rs files

use std::env;			// For parsing command line args
use std::process;		// For exiting program

use minigrep::Config;	// Import our Config struct


fn main() {
	// // Turn args iterator into vector via collect
	// let args: Vec<String> = env::args().collect();

	// // Parses the command line arguments into Config struct
	// let config = Config::new(&args).unwrap_or_else(|err| {
	// 	eprintln!("Problem parsing arguments: {}", err);
	// 	process::exit(1);
	// });

	// Pass the env::args() iterator directly into the new function
	// which passes ownership of the args iterator.
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	// An alternative way to unwrap the result:
	// let config = Config::new(&args);
	// let config = match config {
	// 	Ok(result) => result,
	// 	Err(error) => {
	// 		println!("Problem parsing arguments: {}", error);
	// 		process::exit(1);
	// 	},
	// };

	// Call the logic of the program. If there is an error,
	// return the error and exit the program. We use this pattern
	// instead of unwrap_or_else() since the run function does not
	// return anything we wish to unwrap in the success case.
	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);

		process::exit(1);
	}
}



















