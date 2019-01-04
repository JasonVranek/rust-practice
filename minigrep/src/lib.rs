use std::fs;				// For handling files
use std::error::Error;		// For returning Box<Error>
use std::env;				// For checking Environment variable


pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new (mut args: std::env::Args) -> Result<Config, &'static str> {
		if args.len() < 3 {
			// This is the &'static str case in the function definition Result
            // Since it is static, it doesn't have to be made at runtime.
			return Err("not enough arguments");
		}
        // Consume the item in iterator that is the file name
        args.next();

        // Match over the result when consuming the iterator which is of type Option
		let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

		let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

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
	// lines() separates the text by newline characters into an iterator
    // filter removes all lines that don't contain our query string
    // collect turns this iterator back into a vector of string slices
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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





















