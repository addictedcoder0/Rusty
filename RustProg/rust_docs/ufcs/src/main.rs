trait Foo{
	fn f(&self) -> i32;
}
trait Bar{
	fn f(&self) -> i32;
	fn q(&self){
		println!("method 'q' from 'Bar'");
	}
}
struct Tester;
impl Foo for Tester{
	fn f(&self) -> i32{10}
}
impl Bar for Tester{
	fn f(&self) -> i32{20}
}
impl Tester{
	fn q(&self){
		println!("method 'q' from Tester");
	}
}
fn main() {
	let t = Tester;
	//resolving inheritance level ambiguity
	println!("{:?}",t.q());
	println!("{:?}",<Tester as Bar>::q(&t));
	//resolving Diamond Problem
	println!("{:?}",<Tester as Foo>::f(&t) );
	println!("{:?}",<Tester as Bar>::f(&t) );
}
