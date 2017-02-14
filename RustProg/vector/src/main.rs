fn main() {
    let mut v:Vec<i32> = vec![1,2,3,4];
    println!("third element is : {}",v[2] );

    // v[i] : type of i must be usize.
    

    //println!("trying to access a out of scope index {}",v[8] );

    // how to handle the "index out of bounds" issue
    println!("trying to access a out of scope index :{} ",7);
    match v.get(8) {
    	Some(num) => println!("num : {}",num ),
    	None => println!("sorry the vector is too short"),
    }
  
   // =============== Iterators ========================

   for i in &v{
   	println!("A reference to {}",i);
   } 
	
   for i in &mut v{
   	println!("A mutable reference to {}",i );
   }

   for i in v {
   	println!("Take ownership of the vector and its element {} ",i);
   }
   // after taking the ownership of a vector you can iterate through it only once .
   //but by taking the reference we can iterate as many times we want .
}

