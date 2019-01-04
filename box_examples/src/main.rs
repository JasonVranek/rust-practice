
///////// Implementing the drop trait //////////

struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data: {}", self.data);
	}
}


fn main() {
	// When s goes out of scope, it's custom drop function is called which prints.
	let h = CustomSmartPointer{data: String::from("hello")};
	let w = CustomSmartPointer{data: String::from("world")};
	// w.drop()  //illegal

	// I can free up memory before the scope ends by calling drop(). But
	// if I tried to access the drop function implemented for the struct,
	// the compiler won't let you. This is because it will automatically be
	// called at the end of it's scope, and if it were freed earlier this 
	// would cause a double free error.
	drop(w);
}
