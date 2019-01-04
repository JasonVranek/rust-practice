fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("Do I own {}?", s); // Will give a move error since s was moved into the func
    
    let s2 = String::from("hello");

    // Return values can transfer ownership, so s2 loses ownership to the take_and_giveback func,
    // then the func returns a String that is assigned to s3. 
    let s3 = take_and_giveback(s2);
    println!("s2 moved ownership to s3: {}", s3);

    // If this is uncommented, there will be a move error
    // println!("s2 has ownership still? : {}", s2);

}


fn takes_ownership(some_string: String) {
	println!("I now own {}", some_string);
}

// Takes ownership as a parameter but returns ownership back
fn take_and_giveback(some_string: String) -> String {
	some_string
}