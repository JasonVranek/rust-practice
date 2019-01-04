// The RefCell<T> type represents single ownership over the data it holds
// unlike the Rc<T> which allows multiple ownership of immutable data. The
// invariants for RefCell<T> are enforced at runtime and not compile time.
// This is useful if the compiler can't guarantee you're borrowing correctly
// but you can. Box<T> and RefCell<T> are very similar, but are enforced at 
// compile time and runtime respectively.

// I can be useful to have methods that iternally mutate data, while 
// maintatining immutability to other external functions. 


fn main() {
    println!("Hello, world!");
}
//