use std::io;
fn main(){
	println!("Enter your first number!");
	
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input.");
	
	let x: i32 = input.trim().parse().expect("Invalid input.");
	
	println!("Enter your second number!");
	
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input.");
	
	let y: i32 = input.trim().parse().expect("invalid input");
	
	println!("Your sum is {:?}", x + y);
}