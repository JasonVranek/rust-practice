// Since List has a variant that is recursive, Rust cannot figure out
// how much space it needs to allocate for a List. The solution is to 
// use indirection to store a pointer to the data instead of directly
// storing the data. This means we will wrap the List with a Box<T>, which
// since it is a pointer, has a known a known size to the compiler.
#[derive(Debug)]
enum List {
	Cons(i32, Box<List>),
	Nil,
}


use crate::List::{Cons, Nil};


fn main() {
	let list = Cons(1, 
		Box::new(Cons(2, 
			Box::new(Cons(3, 
				Box::new(Nil))))));
	println!("{:?}", list);

}
