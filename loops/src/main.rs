fn main() {
    println!("Hello, world!");
    for_loop();
    while_loop();
    loop_loop();
    retry_loop();
    for_range();
}


fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
    	println!("the value is: {}", element);
    }
}


fn for_range() {
    for number in (1..4).rev() {
    	println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}


fn while_loop() {
    let mut number = 3;
    while number >= 0 {
    	println!("While loop: {}", number);
    	number -= 1;
    }

}

fn loop_loop() {
	let mut number = 3;
	loop {
		println!("Infinite loop: {}", number);
		number -= 1;
		if number == 0 {
			println!("Breaking out! : {}", number);
			break
		}
	}
}

// If you break out of a loop, you can return its 
// value to a variable as shown below:
fn retry_loop() {
	let mut counter = 0;
	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};
    println!("Reults is :{}", result);
    assert_eq!(result, 20);
}