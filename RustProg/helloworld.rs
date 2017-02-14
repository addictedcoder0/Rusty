
use std::{i32};
fn main() {
	println!("hello world");	

	let num:i32 = 32;
	println!("num : {}",num);

	let is_it_true:bool = true;
	println!("is it true : {}",is_it_true );
	
	// assigning mulitple value
	let (fname,lname) = ("abhishek","srivastava");
	println!("fname : {} , lname :{}",fname,lname );

	//converting into hex , binary and octal value
	println!("binary: {:b} , hex: {:x} , octal: {:o}",5,15,8 );

	


}