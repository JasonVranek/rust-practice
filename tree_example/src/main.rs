use std::rc::{ Rc, Weak };
use std::cell::RefCell;


// Want Node to own it's children, so each child in the Vec is of Rc<T>.
// We want to modify the children so we wrap with RefCell<T> to take ownership.
// Note the parent field cannot be of type Rc<T> because this will contain a ref cycle.
#[derive(Debug)]
struct Node {
	value: i32,
	children: RefCell<Vec<Rc<Node>>>,
	parent: RefCell<Weak<Node>>,
}



fn main() {

	// Create a node with no children
    let leaf = Rc::new(Node {
    	value: 3,
    	children: RefCell::new(vec![]),
    	parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong: {}, leaf weak: {}", 
    	Rc::strong_count(&leaf),
    	Rc::weak_count(&leaf));

    // Start of inner scope
    {
	    // Create a node that contains the leaf as a child by cloning it.
	    // Now, the node in leaf has 2 owners: leaf and branch.
	    let branch = Rc::new(Node {
	    	value: 5,
	    	children: RefCell::new(vec![Rc::clone(&leaf)]),
	    	parent: RefCell::new(Weak::new()),
	    });

	    // Make the branch a parent of the leaf
	    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

	    // This print doesn't result in an infinite cycle meaning we
	    // avoided creating a reference cycle.
	    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

	    println!("leaf strong: {}, leaf weak: {}", 
    		Rc::strong_count(&leaf),
    		Rc::weak_count(&leaf));

	    println!("branch strong: {}, branch weak: {}", 
    		Rc::strong_count(&branch),
    		Rc::weak_count(&branch));

	}
	// End of inner scope
	println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
	println!("leaf strong: {}, leaf weak: {}", 
    		Rc::strong_count(&leaf),
    		Rc::weak_count(&leaf));

}
