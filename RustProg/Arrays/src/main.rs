fn main() {
 let aliens = ["rex","dripto","xrill","cryioon"];
 println!("aliens list :{:?}",aliens );
 // we can't add any element into this 
 let a = aliens[2];
 println!("3rd alien is :{:?}", a);
 println!("last alien is :{:?}", aliens[aliens.len()-1]);
 let arr_pointer = &aliens;
 println!("aliens list is :{:?}",arr_pointer);
 println!("first alient is :{:?}",arr_pointer[0]);
 // using _ b/c this var is not going to be used
 for _ in 0..aliens.len(){ 
 	// this approach demands rust to check the memory 
 	//bound with each element's access
 	// this is inefficient approach .
 }
 for _ in aliens.iter(){
 	// this is efficient approach .
 }
 for _ in arr_pointer{
 	// this is the smallest and nice approach possible
 }

let location = "middle-earth";
let planet = &location[7..12];
println!("planet name is : {:?}", planet);
let mut chars:Vec<char> = planet.chars().collect();
chars.sort();
for i in chars.iter(){
	print!("{:?}",i );    // a e h r t
}
println!();
// we can split and collect the string slices into vector
let v:Vec<&str> = "i am hungary".split(' ').collect();
for i in v.iter(){
println!("{:?}", i);
}
println!("= split as per the usecase =");
// split() accept the closure , 
// which can be defined based on the use cases we have
let v:Vec<&str> = "2io2b4e".split(|c:char|c.is_numeric()).collect();
for i in v.iter(){
println!("{:?}", i);
}
println!("=greeting : string to chars =");
let greeting:Vec<char> = "Hello,世界世!".chars().collect();
for i in greeting.iter(){
	println!("{:?}", i);
}
println!("= greeting : string to bytes =");
let greeting = "Hello,世界世!".bytes();
for i in greeting{
	println!("{:?}", i as char);
}
}

