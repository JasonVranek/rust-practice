fn main() {
    let s1 = String::from("Hello");

    // Here we pass an immutable reference
    let len = calculate_length(&s1);
    // Because we only passed a reference into the function, s1 is still in scope
    println!("The length of '{}' is {}.", s1, len);
    // println! silently borrows are reference even if you don't explicitly pass it one
    println!("The length of '{}' is {}.", &s1, &len);

    let mut s2 = format!("Hello");
    // Here we can create a mutible reference in the scope of s2 (_ to suppress warning)
    let _s3 = &mut s2;
    let _s4 = &mut s2;
    // println!("Mut test: {}, {}.", _s3, _s4);  // This line causes an error
    // you can only have one mutable reference to a particular piece of data in a particular scope!
    let new_len = mutable_reference(&mut s2);
    println!("The new length of '{}' is {}.", s1, new_len);

    string_slice();
    let long_string = String::from("Poop fart butt");
    let first_word = first_word_in_str(&long_string);
    println!("First word in '{}' is {}", long_string, first_word);
}

// This function only takes a reference to an object instead of ownership
fn calculate_length(word: &String) -> usize {
	// word.push_str("adfasf"); // Uncommenting this would be cannot borrow immutable borrowed content
	// since we have an immutable reference which is read-only.
    word.len()
    // Since word does not own what it refers to, so leaving scope does not affect it. It BORROWED!
}

fn mutable_reference(word: &mut String) -> usize {
	// Here because word is a mutable reference, it can be modified without being owned.
    word.push_str(", World!");
    word.len()
}


fn string_slice(){
	let s = String::from("Hello World!");
	let len = s.len();
	let hello = &s[0..=4];
	// let hello = &s[0..5];	// Equivalent
	// let hello = &s[..5];		// Equivalent
	// let world = &s[6..];
	let world = &s[6..len];
	println!("{}, {}", hello, world);
}

fn first_word_in_str(s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' { return &s[..i] }
	}
	return &s[..]
}












