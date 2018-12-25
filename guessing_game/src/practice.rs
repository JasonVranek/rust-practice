use std::io;
use std::cmp::Ordering;
use rand::Rng;
// We can use anything in the rand crate by using rand::
// the Rng trait defines the methods that the random num generators implement


fn main() {
    println!("Guess the number!");

    // Here we use the rand::thread_rng function which looks at the 
    // current thread and is seeded by the OS. gen_range is a method on our
    // thread_rng random number generator.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

	    println!("Please input your guess.");

	    // Initializes an empty mutable String type
	    // :: means new is an associated function of String type (static method)
	    let mut guess = String::new();

	    // Calls the stdin function which comes from the standard library, io library
	    // read_line is a function from the std::io::stdin instance which is a type that
	    // represents the handle to the stdin input from the terminal.
	    // The return type is io::Result, which is an enum that encodes error
	    // handling information. Expect causes your program to crash and display the message
	    // if it receives a Result of type Err as opposed to Ok.
	    io::stdin().read_line(&mut guess)
	        .expect("Failed to read line");

	    // Since guess is initialized as a mutable String, we need to convert it
	    // to an integer type, u32 in this case by shadowing it with an immutable
	    // u32 guess. Trim removes whitespace and newlines. Parse will parse a string
	    // into a number whose type must be specified. If parse is successful it will
	    // return a Ok variant of Result otherwise .except will catch the Err. 
	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => {
	    		println!("Please enter a valid number...");
	    		continue;
	    	}
	    };

	    println!("You guessed: {}", guess);

	    match guess.cmp(&secret_number) {
	    	Ordering::Less => println!("Too small!"),
	    	Ordering::Greater => println!("Too big!"),
	    	Ordering::Equal => {
	    		println!("You win!");
	    		break;
	    	}
	    }
	}
}
