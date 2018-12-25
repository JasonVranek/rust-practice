use std::fs;				// For handling files
use std::error::Error;		// For returning Box<Error>
use std::env;				// For checking Environment variable


pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new (args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			// This is the &'static str case in the function definition Result
			return Err("not enough arguments");
		}

		// Cannot move element unless it is a copied slice since args is reference,
		// this is simpler than using lifetimes, but less efficient at runtime. 
		let query = args[1].clone();
		let filename = args[2].clone();

		// Check for environment variable CASE_INSENSITIVE that is wrapped
		// as a Result. The Result will be Ok() if the Env variable is set, and
		// err if it isn't. (if it is an error, it will return true which means it is unset)
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
	// Adding the '?' will return the error value from the current function
	// for the caller of the function to handle.
	let contents = fs::read_to_string(config.filename)?;

	// Alternatively with no '?' above, we could handle the error as such:
    // let contents = match contents {
    // 	Ok(text) => text,
    // 	Err(error) => {
    // 		panic!("There was a problem reading the file: {:?}", error)
    // 	},
    // };

    let results = if config.case_sensitive {
    	search(&config.query, &contents)
    } else {
    	search_case_insensitive(&config.query, &contents)
    };
 
 	// search() returns a vec and we iterate over it printing the results
    for line in results {
    	println!("{}", line);
    }

    // By default return Ok() type if no error
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	// The lifetime 'a specifies contents lives as long as the return value
	let mut results = Vec::new();
	// The lines method returns a line by line iterator for strings
	for line in contents.lines() {
        if line.contains(query) {
        	results.push(line);
        }
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	let query = query.to_lowercase();
	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
    	let query = "duct";
    	let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

    	assert_eq!(
    	    vec!["safe, fast, productive."],
    	    search(query, contents)
    	    );
    }

    #[test]
    fn case_sensitive() {
    	let query = "rUsT";
    	let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    	assert_eq!(
    	    vec!["Rust:", "Trust me."],
    	    search_case_insensitive(query, contents)
    	    );
    }
}





















