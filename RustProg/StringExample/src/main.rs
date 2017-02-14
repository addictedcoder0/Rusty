fn string_utf8(k:&str){
	for b in k.as_bytes(){
		print!("{},",b);
	}
	println!("");
	for c in k.chars(){
		print!("{},",c);
	}
	println!("");
	//for accessing individual char use 
	//chars().nth(x)=>Option().
	println!("1st char is :{}",k.chars().nth(0).unwrap() );
}
fn main() {
	let greeting = "hello there";
	/* "hello there" is a string literal and its type
	is &'static str.A string literal is a string slice
	that is statically allocated, meaning that it's saved
	inside our compiled program and exist for the entire
	duration it runs.
	*/
	// string literal form1:contains \n and spaces
	let str_literal1 = "hello
	      world";
	// string literal form2:contains \ and trims
	//new line and spaces
	let str_literal2 = "hello\
	      world";     
	println!("string literal form1:{:?}", str_literal1);
	println!("string literal form2:{:?}", str_literal2);      

	//String type
	let s:String = "hello-world".to_string();
	//s.push_str("i am here");
	println!("s : {:?}",s );
	
	let k ="rudra";
	//println!("index 1:{:?}",k[0] ); : error => str can't be indexed.
	let hachiko = "忠犬ハチ公";
	string_utf8(k);
	string_utf8(hachiko);
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
