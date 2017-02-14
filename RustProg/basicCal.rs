
use std::{i32};
fn main() {
	println!("hello world");	

	// Basic Calculations :

	println!("5+4 = {}",5+4);
	println!("5-4 = {}",5-4);
	println!("5*4 = {}",5*4);
	println!("5/4 = {}",5/4);
	println!("5%4 = {}",5%4);

	let mut neg_4 = -4i32;
	//converting into hex , binary and octal value :
	println!("abs(-4): {}",neg_4.abs());

	// there are many mathematical operations :
	println!("MAX 4,5 = {}",4f64.max(5f64) );
}