// By using RefCell<T> we can have an outwardly immutable List,
// but we can use the methods on RefCell<T> to provide access to
// it's interior mutability so we can modify data when needed.



#[derive(Debug)]
enum List {
	Cons(Rc<RefCell<i32>>, Rc<List>),
	Nil,
}

use crate::List::{ Cons, Nil };
use std::rc::Rc;
use std::cell::RefCell;


fn main() {
	// value can be accessed directly later 
    let value = Rc::new(RefCell::new(5));

    // clone value so a and value both have ownership
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // a was wrapped in an Rc<T>, b and c can refer to a
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // use automatic dereferencing * to dereference Rc<T> -> RefCell<T>,
    // which can then be mutable via borrow_mut()
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

