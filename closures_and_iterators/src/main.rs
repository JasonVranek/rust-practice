use rayon::prelude::*;


fn par_sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
    .map(|&i| i * i)
    .sum()
}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.iter()
    .map(|&i| i * i)
    .sum()
}

fn main() {
    println!("Hello, world!");
    let mut nums = Vec::new();
    for x in 0..100 {
    	nums.push(x);
    }
    let sum = par_sum_of_squares(&nums);
    println!("{}", sum);
}
