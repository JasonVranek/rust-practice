use std::ops::Deref;

#[test]
fn deref_box() {
	let x = 5;
    let y = Box::new(x);
    
    assert_eq!(x, *y);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

// MyBox is a tuple struct with one generic element. We need to
// implement the Deref Trait to be able to use dereference MyBox.
impl<T> Deref for MyBox<T> {
	// Defines an associated type for the Deref trait to use.
	type Target = T;

	fn deref(&self) -> &T {
		// Returns a reference to the only element in the tuple
        &self.0
	}
}

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

#[test] 
fn test_mybox() {
	let x = 5;
	let y = MyBox::new(x);

	assert_eq!(5, x);

	// The compiler is running *(y.deref())
	assert_eq!(5, *y);
}

#[test]
fn test_deref_coercion() {
	let s = MyBox::new(String::from("hello"));
	hello(&s)
}

// This accepts an argument which is a reference to MyBox<String>.
// Because we implemented the Deref trait, Rust can convert &MyBox<String>
// into &String by calling deref(). 
fn hello(name: &str) {
	println!("Hello {}", name);
}


































