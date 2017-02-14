use std::io::Write;
use std::str::FromStr;

fn gcd(mut x:u32,mut y:u32) -> u32{
	assert!(x!=0 && y!=0);
	while x!=0{
		if x<y {
			let temp:u32 = x;
			x=y;
			y=temp;
		}
		x = x%y;
	}
	y
}

#[test]
fn test_gcd() {
	assert_eq!(gcd(2*3*11*17,5*7*13*19),1);
	assert_eq!(gcd(2*3*11*17,5*3*13*11),3*11);
}
fn main() {
	//commmand-line argument handling
	let mut numbers =  Vec::new();
	//skip(1) is skipping the program name.
	for arg in std::env::args().skip(1){
		//arg is of type 'str'. require conversion.
		numbers.push(u32 :: from_str(&arg).expect("error parsing the input"));
		//from_str returns a Result type. 
		//Err(e) => Result's expect() will be executed.
		//Ok(v) => Result's expect() will return 'v'.
	}
	if numbers.len() == 0{
		writeln!(std::io::stderr(),"Usage : gcd Number ...").unwrap();
		// exiting the programm with a error:status code. 
		std::process::exit(1);
	}
	let mut d = numbers[0];
	for m in &numbers[1..]{
		// on using numbers , for loop will take ownership
		//&numbers[1..] is a slice of our vector.
		d = gcd(d,*m);
	}
	println!("GCD of numbers : {:?} is {:?}",numbers,d );
	//finding GCD of 2 numbers.
    println!(" GCD : ");
    let x:u32 = 26;
    let y:u32 = 4;
    println!("x:{:?} and y:{:?} => {:?}",x,y,gcd(x,y) ); 
}

