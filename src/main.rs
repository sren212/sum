use std::io;
fn main(){
	// take in the first input
	println!("Enter your first number!");
	
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input.");
	
	let x: i32 = input.trim().parse().expect("Invalid input.");
	
	// take in the second input
	println!("Enter your second number!");
	
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input.");
	
	let y: i32 = input.trim().parse().expect("invalid input");
	
	// output the sum of x and y
	println!("Your sum is {:?}", x + y);
}