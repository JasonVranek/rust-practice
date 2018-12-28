use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
	// A value to represent workout intensity
	let simulated_user_value = 10;

	// A value to generate variety in workout plans
	let simulated_random_number = 7;

	// hashmap_generate_workout(
	// 	simulated_user_value,
	// 	simulated_random_number
	// );

	test_generic_hash_cacher();
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
	println!("starting slow calculation");
	thread::sleep(Duration::from_secs(2));
	println!("finished slow calculation");
	intensity
}

// Generates a workout without closures
fn naive_generate_workout(intensity: u32, random_number: u32) {
	if intensity < 25 {
		println!("Today, do {} pushups!", 
			simulated_expensive_calculation(intensity)
		);
		println!("Next, do {} situps!", 
			simulated_expensive_calculation(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today!");
		} else {
			println!("Today, run for {} minutes!",
				simulated_expensive_calculation(intensity));
		}
	}

}

// Generates a workout using a closure. The closure is still called multiple times.
fn naive_closure_generate_workout(intensity: u32, random_number: u32) {
	// This closure statements contains the definition of an anonymous function,
	// and not the resulting value when it is called.
	let expensive_closure = |num: u32| -> u32 {
		println!("starting slow calculation");
		thread::sleep(Duration::from_secs(2));
		println!("finished slow calculation");
		num
	};
	if intensity < 25 {
		println!("Today, do {} pushups!", 
			expensive_closure(intensity)
		);
		println!("Next, do {} situps!", 
			expensive_closure(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today!");
		} else {
			println!("Today, run for {} minutes!",
				expensive_closure(intensity)
			);
		}
	}

}

// This struct holds a closure and an optional result value. The 
// calculation field is of generic type T and specifies that it's a 
// closure by using the Fn trait bound. Structs require knowing the types of each
// of it's fields, so this closure must accept only one u32 parameter. Before the
// closure is executed, value will be None. When the code asks for the value of result, 
// the closure will execute and the result will be a Some variant. Subsequent calls to
// value will return the cached result held in the value field.
struct Cacher<T> where T: Fn(u32) -> u32 {
	calculation: T,
	value: Option<u32>,
}

impl<T> Cacher <T> where T: Fn(u32) -> u32 {
	// Create a Cacher instance that contains the closure in the calculation field.
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
			value: None,
		}
	}

	// Check whether we have already calculated the closure
	fn value(&mut self, arg: u32) -> u32 {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.calculation)(arg);
				self.value = Some(v);
				v
			},
		}
	}
}

// Generates a workout using a closure contained in a struct.
fn generate_workout(intensity: u32, random_number: u32) {
	// Create a new Cacher instance and pass in a closure as an argument. The closure
	// will be run a maximum of one time, but the result of it can easily be reused without
	// redundant code.
	let mut expensive_result = Cacher::new(|num: u32| -> u32 {
		println!("starting slow calculation");
		thread::sleep(Duration::from_secs(2));
		println!("finished slow calculation");
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", 
			expensive_result.value(intensity)
		);
		println!("Next, do {} situps!", 
			expensive_result.value(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today!");
		} else {
			println!("Today, run for {} minutes!",
				expensive_result.value(intensity)
			);
		}
	}
}


struct HashCacher <T> where T: Fn(u32) -> u32 {
	calculation: T,
	value: HashMap <u32, u32>,
}

impl <T> HashCacher <T> where T: Fn(u32) -> u32 {
	fn new(calculation: T) -> HashCacher<T> {
		HashCacher {
			calculation,
			value: HashMap::new(),
		}
	}

	// Only use closure if that key hasn't been called before:
	// fn value(&mut self, arg: u32) -> u32 {
	// 	// The entry method on a HashMap returns an Entry type enum with
	// 	// Occupied and Vacant fields.
	// 	if let Entry::Occupied(o) = self.value.entry(arg) {
			// o.get().clone()
	// 	} else {
	// 		let v = (self.calculation)(arg);
	// 		self.value.insert(arg, v);
	// 		v
	// 	}
	// }
	// Alternative way to handle the Entry enum matching:
	fn value(&mut self, arg: u32) -> u32 {
		match self.value.entry(arg) {
			Entry::Occupied(o) => o.get().clone(),
			Entry::Vacant(_) => {
				let v = (self.calculation)(arg);
				self.value.insert(arg, v);
				v
			}
		}
	}
}


// Generates a workout using a closure contained in a struct. The difference
// now is that the results are stored in a HashMap. This fixes the issue where
// once the struct has been set, it cannot be modified. Now, if a certain key
// hasn't been set yet it will generate it and store it in the HashMap using the 
// closure. Otherwise if a repeated intensity is called, the value will be returned.
fn hashmap_generate_workout(intensity: u32, random_number: u32) {
	// Create a new Cacher instance and pass in a closure as an argument. The closure
	// will be run a maximum of one time, but the result of it can easily be reused without
	// redundant code.
	let mut expensive_result = HashCacher::new(|num| {
		println!("starting slow calculation");
		thread::sleep(Duration::from_secs(2));
		println!("finished slow calculation");
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", 
			expensive_result.value(intensity)
		);
		println!("Next, do {} situps!", 
			expensive_result.value(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today!");
		} else {
			println!("Today, run for {} minutes!",
				expensive_result.value(intensity)
			);
		}
	}
}

// C: The closure (MUST BE OF AT LEAST ONE TYPE: Fn, FnMut, FnOnce)
// K: The input of the closure and the key that locates result in HashMap
// V: The output of the closure and the value that is stored at key K in the HashMap
struct GenericHashCacher <C, K, V> 
	where C: FnMut(K) -> V,
	      K: std::hash::Hash + std::cmp::Eq + std::clone::Clone,
	      V: std::clone::Clone
{
	calculation: C,
	value: HashMap <K, V>,
}

// All closures
impl <C, K, V> GenericHashCacher <C, K, V> 
	where C: FnMut(K) -> V,
	      K: std::hash::Hash + std::cmp::Eq + std::clone::Clone,
	      V: std::clone::Clone
{

	fn new(calculation: C) -> GenericHashCacher<C, K, V> 
		where K: std::hash::Hash + std::cmp::Eq
	{
		GenericHashCacher {
			calculation,
			value: HashMap::new(),
		}
	}

	// Only use closure if that key hasn't been called before:
	fn value(&mut self, arg: K) -> V {
		match self.value.entry(arg.clone()) {
			Entry::Occupied(o) => o.get().clone(),
			Entry::Vacant(_) => {
				let v = (self.calculation)(arg.clone());
				self.value.insert(arg, v.clone());
				v
			}
		}	
	}
}

fn test_generic_hash_cacher() {
	let mut closure_struct = GenericHashCacher::new(move |input| {
		println!("starting slow calculation");
		thread::sleep(Duration::from_secs(2));
		println!("finished slow calculation");
		input
	});

	// let input = 5;
	// closure_struct.value(input);
	// println!("Done with {}", input);

	// let input = 10;
	// closure_struct.value(input);
	// println!("Done with {}", input);

	// let input = 5;
	// closure_struct.value(input);
	// println!("Done with {}", input);

	// let input = "hello";
	// closure_struct.value(input);
	// println!("Done with {}", input);

	// let input = "world";
	// closure_struct.value(input);
	// println!("Done with {}", input);

	// let input = "hello";
	// closure_struct.value(input);
	// println!("Done with {}", input);

	let input = [1, 2];
	closure_struct.value(input);
	println!("Done with {:?}", input);

	let input = [2, 2];
	closure_struct.value(input);
	println!("Done with {:?}", input);

	let input = [1, 2];
	closure_struct.value(input);
	println!("Done with {:?}", input);
}













