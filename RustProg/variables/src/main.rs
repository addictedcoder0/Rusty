static AUTHOR:&'static str = "Rudra";
static MAX_LIMIT:i32 = 10;

fn main() {
	/*
	let num; // error
	let num:i32 ; // accepted
	let num = 10; // good: compiler will infer the type
	let num:i32 = 10; // Best : you helped the compiler.
	*/
	let empty = ();// no value
	let _num:i32 = 10; // use of _ prefix to suppress the warning
	//num = 100; // error , immutable in nature
	let mut mut_num = 10;
	mut_num += 100; // accepted , mutability is provided
    println!("Author:{:?} and MAX_LIMIT :{:?}",AUTHOR,MAX_LIMIT);
    println!("empty has value : {:?}",empty );
    println!("mut_num : {:?}",mut_num );

    let first_name = "gradon";
    let last_name = "Hoare";
    //let fullName = firstName + lastName; // error '+' not supported for &str
    //let full_name = first_name.to_string() + last_name ;
    let full_name = format!("{}{}",first_name,last_name); // as format! returns the string .
    println!("full Name : {:?}",full_name);

    let point:f32 = _num as f32;
    println!("_num:i32 as f32 :{:?}",point ); 
}
