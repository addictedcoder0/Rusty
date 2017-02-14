fn is_odd(x:u32) ->bool{
	x%2==1
}
fn main() {
  println!("find the sum of all squared odd numbers below 1000");
  let upper=1000;
  //Imperative approach
  let mut acc =0;
  for i in 0..{
  	let i_square = i*i;
  	if i_square>1000 {
  		break;
  	}else if is_odd(i_square){
  		acc+=i_square;
  	}  
  }	
 println!("sum via imperative approach : {:?}",acc );

//functional approach 
 let sum:u32 = (0..).map(|n| n*n)
 					.take_while(|&n| n<upper)
 					.filter(|&n| is_odd(n))
 					.fold(0,|sum,i| sum+i);
 println!("sum via functional approach : {:?}",sum ); 	
}
