fn main() {
    println!("Hello, world!");
    another_function(6, false);
    blocks();
    let x = return_value(100);
    println!("x:{}", x);
}

fn another_function(x: i32, y: bool) {
	println!("Another function! x: {}, y: {}", x, y);
}

fn blocks() {
	let x = 5;

	// It is possible to create new scopes in a function
    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}, y: {}", x , y);
}

// Specify return type with ->
fn return_value(x: i32) -> i32{
	// Omitting semicolon causes the line to act as an expression
	// and will thus return its value.
    x
}