fn main() {
//collect : Consumer
let rng = 0..10;
let rngvec:Vec<i32> = rng.collect();
println!("collect operation :{:?}",rngvec); 
let mut rng1 = 0..10;  
//find : Consumer
let odd_rng = rng1.find(|n| n%2!=0); 
println!("first odd : {:?}",odd_rng.unwrap() ); 
//filter : Adapter
let rng2 = 0..10;
let rng_even:Vec<i32> = rng2.filter(|n| n%2==0)
							.collect();
println!("even rngs :{:?}",rng_even);
// cube of even numbers
let rng3 = 0..10;
let rng_even_cube:Vec<i32> = rng3.filter(|n| n%2==0)
				.map(|n| n*n*n).take(3).collect(); 	
println!("even rngs cube :{:?}",rng_even_cube );
let sum = (0..100).fold(0,|sum,n| {n+sum});
println!("sum of (0..100) :{:?}",sum );
//exer: calculate the product of all the cubes 
//of the integers in the range 1..6
let product = (1..6).map(|n| n*n*n)
				.fold(1,|product,n| {product*n});
println!("product is : {:?}",product );
}
