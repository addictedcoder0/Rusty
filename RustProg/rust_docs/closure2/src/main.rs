fn apply<F>(f:F) where F:Fn(){
	f();
}
fn function(){
	println!("I am a function");
}
fn main() {
/*
Type Anonymity : when a closure is defined , the compiler
implicitly creates a new anonymous structure to store the 
capture variables inside,since the new type is of unknown
type any usage in the function will require generics.
however <T> will be ambiguous and not allowed.
thus bounding by Fn,FnMut or FnOnce is sufficient .
*/
let x:i32=7;
let name="hello-world";
let print = || {
	println!("x:{:?} and name:{:?}",x,name );
}; 
apply(print);//print is moved here
apply(print); => error :moved value 
//a fn ,satisfy the trait bound of closure can be passed.
apply(function);
}
