#[test]
fn iter_demonstrator() {
	let v1 = vec![1, 2, 3];
	// Calling iter() returns immutable references
	let mut v1_iter = v1.iter();

	assert_eq!(v1_iter.next(), Some(&1));
	assert_eq!(v1_iter.next(), Some(&2));
	assert_eq!(v1_iter.next(), Some(&3));
	assert_eq!(v1_iter.next(), None);

}

/////////////////////////////////////////////////////////////////

#[test]
fn into_iter_demonstrator() {
	let v1 = vec![1, 2, 3];
	// Calling into_iter() returns mutable references
	let mut v1_iter = v1.into_iter();

	assert_eq!(v1_iter.next(), Some(1));
	assert_eq!(v1_iter.next(), Some(2));
	assert_eq!(v1_iter.next(), Some(3));
	assert_eq!(v1_iter.next(), None);

}

/////////////////////////////////////////////////////////////////

#[test]
fn iterator_sum() {
	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();

	// Calling sum on an iterator consumes it
	let sum: i32 = v1_iter.sum();
	assert_eq!(sum, 6);

	// v1_iter was moved after the sum, so this will fail:
	// println!("{:?}", v1_iter);
}

/////////////////////////////////////////////////////////////////

#[test]
fn map_iterator() {
	let v1 = vec![1, 2, 3];

	// Update contents of iterator using map with closure. This lets
	// us specify any operation to be performed on each item in an iterator.
	let updated_iter = v1.iter().map(|x| x + 1);

	// Since iterators are lazy, we collect it back into a vector
	let consumed: Vec<i32> = updated_iter.collect();
	assert_eq!(consumed, [2, 3, 4]);
}

/////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

// filter() removes items from the iterator if they 
// fail the Boolean condition inside the closure.
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter()						// mutable ref iter
	    .filter(|s| s.size == shoe_size)	// each <Shoe> item is s
	    .collect()							// collect iterator back to vector
}

#[test]
fn filters_by_size() {
	let shoes = vec![
        Shoe { size: 10, style: format!("sneaker") },
        Shoe { size: 13, style: format!("sandal") },
        Shoe { size: 10, style: format!("boot") },
	];

	let shoes_my_size = shoes_in_my_size(shoes, 10);

	assert_eq!(shoes_my_size, 
		vec![ Shoe { size: 10, style: format!("sneaker") },
        Shoe { size: 10, style: format!("boot") },]
        );
}


/////////////////////////////////////////////////////////////////

struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter { count: 0 }
	}
}

impl Iterator for Counter {
	// Set the associated type for iterator to u32
	type Item = u32;	

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;

		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}

#[test]
fn call_next_on_counter() {
	let mut counter = Counter::new();
	assert_eq!(counter.next(), Some(1));
	assert_eq!(counter.next(), Some(2));
	assert_eq!(counter.next(), Some(3));
	assert_eq!(counter.next(), Some(4));
	assert_eq!(counter.next(), Some(5));
	assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
	// skip: skips the first item [2,3,4,5]
	// zip: combines the two iterators into a tuple (1,2),(2,3)...
	// NOTE: zip produces 4 pairs (1,2)(2,3)(3,4)(4,5) and omits (5, None) 
	//       since it will return None if either input is None
	// map: multiples the items in each tuple [2,6,12,20]
	// filter: returns the items divisible by 3
	// collect: turns the iterator back into a vector
	let result: Vec<_> = Counter::new().zip(Counter::new().skip(1))
	                        .map(|(a, b)| a * b )
	                        .filter(|x| x % 3 == 0)
	                        .collect();
	assert_eq!(result, vec![6, 12]);
    let sum: u32 = result.into_iter().sum();
    assert_eq!(sum, 18);
}





























