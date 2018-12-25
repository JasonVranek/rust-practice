fn main() {
    let number = 8;

    if number < 5 {
    	println!("condition was true");
    } else if number == 7 {
    	println!("The number is 7");
    } else {
    	println!("condition was false");
    }

    if_let();
}

fn if_let() {
	let condition = false;
	// You can have if statements in variable assignments, however
	// if "six" was the return in the else, the compiler would give an error
	let number = if condition {
		5
	} else {
		6
	};

	println!("The value of number is: {}", number);
}
