use std::io;
fn main() {
    let mut input = String:: new();
    /*
    rust asks us to be prepared in case if the value 
    is not provided.
    rust returns the result type ,we use ok() to convert
    Result -> Option (it contains how many bytes were read)
    we use expect("panic msg") in case of err.   
    */
    println!("String : ?");
    io::stdin().read_line(&mut input).ok().expect("not able to read");
    // using trim() to remove /n or " "
    println!("String : {:?}",input.trim() );
    println!("number : ?");
    input.clear();
    io::stdin().read_line(&mut input).ok().expect("not able to read");
    let num:Result<u32,_> = input.trim().parse();
    //println!("number : {:?}",num.unwrap() );
    let number = match num{
    	Ok(i) => i ,
    	Err(_) => 0
    };
    println!("number read using match :{:?}",number );
    

    println!("enter the magical number");
    input.clear();
    io::stdin().read_line(&mut input).ok().expect("not able to read");
    let magical_number:Result<u32,_> = input.trim().parse();
    let magic:u32 =	match magical_number{
    			Ok(j) => j,
    			Err(_) => 0
    		}; 
    match magic{
    	1 => println!("unity"),
    	2|3|5|7|11 => println!("prime"),
    	_ => println!("nothing magical")
    };		


    println!("enter the magician name : ");
    input.clear();
    io::stdin().read_line(&mut input).ok().expect("not able to read");
    let magician_res:&str = input.trim();
    match magician_res{
    			"xeron" => println!("best one"),
    			"saron" => println!("ya he is kind of ok"),
    			_ => println!("who ? never heard of him")
    		}; 
    

    let god_desc = ("thor" , true ,1500u32);
    match god_desc {
    	(name,demi,_) if demi => {
    		println!("this is demigod : {:?}",name );
    							},
    	(_,_,power) if power>1000 => {
    		println!("he posses power equivalent to {:?} dinos",power );
    							},
    	_ => println!("its hard to tell ")
    };

}
