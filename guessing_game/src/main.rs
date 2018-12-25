use std::io;
use std::cmp::Ordering;
use rand::Rng;









fn main() {

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("Secret nuum: {}", secret_number);

	loop {

		let mut guess = String::new();

	    println!("Guess a number:");

		io::stdin().read_line(&mut guess)
		        .expect("Failed to read line");

		println!("You guessed {}", guess);

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Error input must be an integer!");
				continue;
			}
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => { println!("Too small!") },
			Ordering::Greater => { println!("Too big") },
			Ordering::Equal => { 
				println!("Got it!");
				break;
			}
		};
	}
}