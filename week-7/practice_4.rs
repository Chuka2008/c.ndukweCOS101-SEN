use std::io;

fn add(a: i32, b: i32) {
	let sum = a + b;

	println!("Sum of A and B = {}",sum);
}

fn main() {

	let mut input1 = String::new();
	println!("Enter input for parameter A:");
	io::stdin().read_line(&mut input1).expect("Not a valid String");
	let side:f32 = input1.trim().parse().expect("Not a valid Number");

	let mut input2 = String::new();
	println!("Enter input for parameter B:");
	io::stdin().read_line(&mut input2).expect("Not a valid String");
	let side:f32 = input2.trim().parse().expect("Not a valid Number");

	// call add function with arguments
	add(a, b)
}