use std::result;
enum ConcreteError{
	foo,
	bar,
}
fn main() {
 type Num = i32;
 let x:i32 = 5;
 let y:Num = 10;
 if x==y{
 	println!("these are equal");
 }else{
 	println!("they aren't equal");
 }
 // type aliases with generics
type Result<T> = result::Result<T,ConcreteError>;
//This creates a specialized version of the Result type,
//which always has a ConcreteError for the E part of Result<T, E>
}
