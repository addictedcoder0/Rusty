fn main() {
	let mut x:i32 = 10;

	println!("Simple Loop : ");
	loop {
		if(x>10 && x%2==0){
			println!("{:?}", x);
			x+=1;
			continue;
		}

		if x>20 {
			break;
		}

		x+=1;
		continue;
	}

	let mut y:i32 = 20;

	println!("While Loop : ");
	while y<25 	{	
		println!("{:?}", y);
		y+=1;
	
	}

	println!("For Loop : ");
	for i in 1..10 {
		println!("i : {}", i);
	}
}