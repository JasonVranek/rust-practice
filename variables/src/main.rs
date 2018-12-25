fn main() {
	// If mut is excluded we have a compile error
    let mut x = 5; 
    println!("The value of x is {}", x);
    x = 6;
    println!("The updated value of x is {}", x);

    // Shadowing => The second variables value is what we get
    // when we use the first variables value:
    let y = 5;
    let y = y + 5;
    let y = y * 10;
    println!("The value of y is {}", y);

    // Shadowing allows us to reuse variable names while letting 
    // us change type as well which cannot be done with a mut:
    let spaces = "        ";
    let spaces = spaces.len();
    println!("The number of spaces is {}", spaces);

    // Constants must have their type annotated, and should be
    // _ separated with all caps. 
    const MY_CONST : i32 = 6;
    println!("The value of MY_CONST is {}", MY_CONST);

    // Typles are fixed length
    let tup: (i32, f64, bool) = (500, 6.9, true);
    let (a, b, c) = tup;
    println!("Destructuring the tuple: {}, {}, {}, {}, {}, {}", a, b, c, tup.0, tup.1, tup.2);

    // Arrays must be of the same type and are fixed length. They are 
    // implemented on the stack instead of the heap.
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 3] = [10, 11, 12];
    println!("{}, {}", a[0], b[2]);
}
