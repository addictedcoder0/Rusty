use std::mem;
fn main() {
	// simple closure :
	/*
    |closure-args|=> define arguments
    {closure-body}=> ops to perform
    */
    let demo = |x:i32|{println!("square of {:?} : {:?}",x, x*x)};
    demo(5);
    demo(10);

    /*
		A closure to print the color(environment var) which immediately borrows('&')
		'color' and stores the borrow and the closure in the "print" variable.
		It will remain Borrowed untill "print" goes out of scope . 
	*/
    let color ="green";
    let print = || println!("color : {:?}",color );
    print();
    print();

    /*
    	A closure to increment 'count' could take either '&mut count'
    	or 'count' but '&mut count' is less restrictive so it takes 
    	that . Immediately borrows 'count' .
    	A mut is required on 'inc' b/c a '&mut' is stored inside.
    	calling the closure here mutates the closure which requires
    	a 'mut'. 
    */
    let mut count =0;
    let mut inc = ||{
    	count+=1;
    	println!("count : {:?}",count );
    };
    inc();
    inc();
    //let reborrow = &mut count;=>error:b/c mut ref 'inc' is inscope 

    /*
    	non-copy type when interacting with closure , it must move
    	so movable immediately moves into the closure.
    */
    let movable = Box::new(3);
    let consume = || {
    	println!("movable : {:?}",movable );
    	// an operation that moves the ownership.
    	let x = movable;
    };
    consume();
    //consume(); => error , value moved
    
    let nums = vec![1,2,3,4];
    let takes_num = || {
    //no ownership moving operation, then it is a ref.	
    	println!("nums : {:?}",nums)
    };
    takes_num();
    takes_num();
	println!("nums : {:?}",nums );

	let nums2 = vec![1,2,3,4];
	let takes_num_with_move =  || {
		println!("nums2 : {:?}",nums2);
	// ownership moving operation.	
		let x=nums2;
	};
	takes_num_with_move();
	//takes_num_with_move();=>error:value used after move

	
}

