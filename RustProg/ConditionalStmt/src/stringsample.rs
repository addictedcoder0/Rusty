fn main() {
	let sample = "hello there , welcome to Rust!!!";
	println!("length = {:?}", sample.len());

	let (first,second) = sample.split_at(13);
	println!("first = {:?} \n second = {:?}",first,second );

	let count = sample.chars().count();

	// individual characters 
	let mut chars = sample.chars();
	let mut indiv_char = chars.next();
	loop {
		match indiv_char {
			Some(expr) => println!("{}", expr),
			None => break,
		}
		indiv_char = chars.next();
	}
	println!( );
	// individual words 
	let mut words = sample.split_whitespace();
	let mut word_iter = words.next();

	loop {
	 	match word_iter {
	 		Some(expr) => println!("{}",expr ),
	 		None => break,
	 	}

	 	word_iter = words.next();
	 } 

	// multiline string 
	let sample_multiline = "hello world \nis the most \nBasic program \nfor learners";
	let mut lines = sample_multiline.lines();
	let mut line = lines.next();

	loop {
		match line {
			Some(expr) => println!("{}",expr ),
			None => break,
		}
		line = lines.next();
	}

	// finding a String  
	println!("sample_multiline contains \"Basic\" = {}",sample_multiline.contains("Basic") );
}