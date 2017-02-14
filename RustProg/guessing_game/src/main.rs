extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number Game : ");
	loop{

		println!("Please input your Guess : ");
		let mut guess = String::new();

		io::stdin().read_line(&mut guess).expect("Failed to read the line");
	   	let secret_num = rand::thread_rng().gen_range(1,10);
	   
		//let guess:u32 = guess.trim().parse().expect(" Please type a Number ");
		   /*
		   Wait a minute, 
		   I thought we already had a guess? We do, but Rust allows us to ‘shadow’ the previous guess with a new one. 
		   This is often used in this exact situation, where guess starts as a String, 
		   but we want to convert it to an u32. Shadowing lets us re-use the guess name, 
		   rather than forcing us to come up with two unique names like guess_str and guess, or something else.
		   */
		   
		   // =====================================================================================

		   /*
	   		we modifed the flow of input acceptance in order to handle the exceptional cases,
	   		we took the advantage of the match for getting the input and seeing if it is as per requirement
	   		if not we will continue , else accept the entered input.
	   		*/


	   		/*
	   		This is how you generally move from ‘crash on error’ to ‘actually handle the error’,
by switching from expect() to a match statement. A Result is returned by parse(), 
this is an enum like Ordering, but in this case, each variant has some data associated with it:
Ok is a success, and Err is a failure. Each contains more information: the successfully parsed integer,
or an error type. In this case, we match on Ok(num), which sets the name num to the unwrapped Ok value (the integer),
and then we return it on the right-hand side. In the Err case, we don’t care what kind of error it is,
so we just use the catch all _ instead of a name. This catches everything that isn't Ok,
and continue lets us move to the next iteration of the loop; in effect,
this enables us to ignore all errors and continue with our program.
	   		*/
	   	let guess:u32 = match guess.trim().parse() {
	   		Ok(input) => input,
	   		Err(_)  => continue,
	   	};

		match guess.cmp(&secret_num){
		    Ordering::Less  => println!("Too Small !!!"),
		    Ordering::Equal  => {println!("you won !!!");break;}
		    Ordering::Greater  => println!("Too Big !!!"),
		}	
	}
   
}
