struct User {
	email: String,
	username: String,
	active: bool,
	sign_in_count: u64,
}

// Tuple struct examples:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// Since the function parameters email and username are the same as the email
// and username fields in the struct, we can use field init shorthand and omit
// email: and username:
fn build_user(email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}


fn main() {
    println!("Hello, world!");
    let user = build_user(format!("jvranek@ucsc.edu"), format!("jvranek"));
    println!("The username: {}", user.username);

    // We can use struct update syntax to easily update a user:
    let user2 = User {
    	email: format!("jasonvranek@gmail.com"),
    	username: format!("new username!"),
    	..user 
    	// alternatively could have written:
    	// active: user.active,
    	// sign_in_count: user.sign_in_count,
    };
    println!("The new username is: {}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
