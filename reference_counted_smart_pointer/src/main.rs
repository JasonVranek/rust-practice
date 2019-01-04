// We use Rc<T> when we want to allocate some data on the heap to be
// accessed by multiple parts of our program, but we can't determine
// at compile time which part of the program will use the data last.
// Rc<T> is only use in single threaded scenarios!
use std::rc::Rc;

enum List {
	Cons(i32, Rc<List>),
	Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5,
    	Rc::new(Cons(10,
    		Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Both b and c reference the same data: a
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
    	let _c = Cons(4, Rc::clone(&a));
    	println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("counter after c goes out of scope: {}", Rc::strong_count(&a));
}
